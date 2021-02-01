use prometheus::core::Collector;
use prometheus::{unregister, CounterVec, Encoder, GaugeVec, HistogramVec, TextEncoder};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, RwLock};
use std::thread;

const LABELNAME_APPNAME: &str = "appname";
const LABELNAME_SINK_TO: &str = "sink_to";

pub enum MetricType {
    Counter,
    Histogram,
    Gauge,
}

pub struct RegisterMetricInput {
    pub metric_type: MetricType,
    pub metric_name: String,
    pub metric_desc: String,
    pub labels: Vec<String>,
}

pub struct TrackCountInput {
    pub metric_name: String,
    pub count: f64,
    pub labels: HashMap<String, String>,
}

pub struct TrackHistogramInput {
    pub metric_name: String,
    pub value: f64,
    pub labels: HashMap<String, String>,
}

pub struct GatherMetricMsg {}

pub enum TypedTrackInput {
    Counter(TrackCountInput),
    Histogram(TrackHistogramInput),
}

#[derive(Debug)]
pub struct StatsdCollector {
    counter_vec_map: HashMap<String, CounterVec>,
    histogram_vec_map: HashMap<String, HistogramVec>,
    gauge_vec_map: HashMap<String, GaugeVec>,
    app_name: String,
}

// TODO: note that currently only one single instance of StatsdActor could be initialised because it register to the global registry
impl StatsdCollector {
    pub fn new(
        app_name: String,
        metrics: Vec<RegisterMetricInput>,
    ) -> (Arc<RwLock<StatsdCollector>>, Sender<TypedTrackInput>) {
        let (tx, rx) = channel::<TypedTrackInput>();
        let mut collector = StatsdCollector {
            counter_vec_map: HashMap::<String, CounterVec>::new(),
            histogram_vec_map: HashMap::<String, HistogramVec>::new(),
            gauge_vec_map: HashMap::<String, GaugeVec>::new(),
            app_name,
        };

        for metric_input in metrics {
            collector.handle_register_metrics(metric_input)
        }

        let collector_shared = Arc::new(RwLock::new(collector));
        let collector_clone = collector_shared.clone();
        thread::spawn(move || loop {
            match rx.recv() {
                Ok(input) => {
                    let mut x = collector_clone.write().unwrap();
                    x.handle_track_msg(input);
                }
                Err(e) => {
                    warn!(
                        "StatsdCollector receive from rx error {:?}. terminating..",
                        e
                    );
                    return;
                }
            }
        });

        (collector_shared, tx)
    }
}

impl StatsdCollector {
    fn handle_register_metrics(&mut self, msg: RegisterMetricInput) {
        let mut labels_vec: Vec<&str> = msg.labels.iter().map(|x| x.as_str()).collect();
        labels_vec.push(LABELNAME_APPNAME);
        labels_vec.push(LABELNAME_SINK_TO);

        let lebels_sli = labels_vec.as_slice();
        match msg.metric_type {
            MetricType::Counter => {
                self.counter_vec_map.insert(
                    msg.metric_name.clone(),
                    register_counter_vec!(
                        msg.metric_name.clone(),
                        msg.metric_desc.as_str(),
                        lebels_sli
                    )
                    .unwrap(),
                );
            }
            MetricType::Histogram => {
                self.histogram_vec_map.insert(
                    msg.metric_name.clone(),
                    register_histogram_vec!(
                        msg.metric_name.clone(),
                        msg.metric_desc.as_str(),
                        lebels_sli
                    )
                    .unwrap(),
                );
            }
            MetricType::Gauge => {
                self.gauge_vec_map.insert(
                    msg.metric_name.clone(),
                    register_gauge_vec!(
                        msg.metric_name.clone(),
                        msg.metric_desc.as_str(),
                        lebels_sli
                    )
                    .unwrap(),
                );
            }
        };
    }

    fn handle_track_msg(&mut self, msg: TypedTrackInput) {
        match msg {
            TypedTrackInput::Counter(t_msg) => self.handle_track_count(t_msg),
            TypedTrackInput::Histogram(t_msg) => self.handle_track_histogram(t_msg),
        }
    }

    fn handle_track_count(&mut self, msg: TrackCountInput) {
        match self.counter_vec_map.get(msg.metric_name.as_str()) {
            Some(counter_vec) => {
                if counter_vec.desc().len() == 0 {
                    warn!(
                        "Handler TrackCountMsg counter_vec has no desc. metric_name = {}",
                        msg.metric_name.as_str()
                    );
                    return;
                }

                let label_names = &counter_vec.desc()[0].variable_labels;
                let label_values = self.build_label_values(label_names, &msg.labels);

                match counter_vec.get_metric_with_label_values(&label_values[..]) {
                    Ok(counter) => {
                        counter.inc_by(msg.count);
                    }
                    Err(e) => {
                        warn!(
                            "Handler TrackCountMsg get counter for metric_name = {} with labels {:?} failed due to {:?}",
                            msg.metric_name.as_str(),
                            label_values,
                            e,
                        );
                        return;
                    }
                }
            }
            None => {
                info!(
                    "Handler TrackCountMsg counter_vec not found for metric_name = {}",
                    msg.metric_name.as_str()
                );
            }
        }
    }

    fn handle_track_histogram(&mut self, msg: TrackHistogramInput) {
        match self.histogram_vec_map.get(msg.metric_name.as_str()) {
            Some(vec) => {
                if vec.desc().len() == 0 {
                    warn!(
                        "Handler TrackHistogramMsg vec has no desc. metric_name = {}",
                        msg.metric_name.as_str()
                    );
                    return;
                }

                let label_names = &vec.desc()[0].variable_labels;
                let label_values = self.build_label_values(label_names, &msg.labels);

                match vec.get_metric_with_label_values(&label_values[..]) {
                    Ok(counter) => {
                        counter.observe(msg.value);
                    }
                    Err(e) => {
                        warn!(
                            "Handler TrackHistogramMsg get vec for metric_name = {} with labels {:?} failed due to {:?}",
                            msg.metric_name.as_str(),
                            label_values,
                            e,
                        );
                        return;
                    }
                }
            }
            None => {
                info!(
                    "Handler TrackHistogramMsg vec not found for metric_name = {}",
                    msg.metric_name.as_str()
                );
            }
        }
    }

    pub fn handle_gather_metrics(&mut self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        let mut buffer = vec![];
        encoder.encode(&metric_families, &mut buffer).unwrap();
        let res = String::from_utf8(buffer.clone()).unwrap();

        for (_, v) in self.counter_vec_map.iter_mut() {
            v.reset();
        }
        for (_, v) in self.histogram_vec_map.iter_mut() {
            v.reset();
        }

        res
    }

    // temp helper function to enable test
    fn _de_register_vecs(self) {
        for (_, v) in self.counter_vec_map.into_iter() {
            let _ = unregister(Box::new(v));
        }
        for (_, v) in self.histogram_vec_map.into_iter() {
            let _ = unregister(Box::new(v));
        }
    }

    fn build_label_values<'a>(
        &'a self,
        label_names: &Vec<String>,
        label_map: &'a HashMap<String, String>,
    ) -> Vec<&'a str> {
        let mut label_values = vec![];
        for label_name in label_names {
            if label_name == LABELNAME_APPNAME {
                label_values.push(self.app_name.as_str());
                continue;
            }

            let label_value = match label_map.get(label_name) {
                Some(v) => v.as_ref(),
                None => {
                    if label_name == LABELNAME_SINK_TO {
                        ""
                    } else {
                        "default"
                    }
                }
            };
            label_values.push(label_value);
        }

        label_values
    }
}
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn test_track_and_gather() {
//         let metricname_http_req_count = "http_req_count";
//         let labelname_endpoint = "endpoint";
//
//         let mut x = StatsdCollector::new(
//             "test_track_and_gather".to_string(),
//             vec![RegisterMetricInput {
//                 metric_type: MetricType::Counter,
//                 metric_name: metricname_http_req_count.to_string(),
//                 metric_desc: "Number of HTTP requests made.".to_string(),
//                 labels: vec![labelname_endpoint.to_string()],
//             }],
//         );
//
//         x.handle_track_count(TrackCountInput {
//             metric_name: metricname_http_req_count.to_string(),
//             count: 1.0,
//             labels: Default::default(),
//         });
//         x.handle_track_count(TrackCountInput {
//             metric_name: metricname_http_req_count.to_string(),
//             count: 2.0,
//             labels: Default::default(),
//         });
//
//         let mut y = TrackCountInput {
//             metric_name: metricname_http_req_count.to_string(),
//             count: 1.0,
//             labels: Default::default(),
//         };
//         y.labels
//             .insert(labelname_endpoint.to_string(), "matrix".to_string());
//         x.handle_track_count(y);
//
//         let res = x.handle_gather_metrics();
//         let expected = r#"# HELP http_req_count Number of HTTP requests made.
// # TYPE http_req_count counter
// http_req_count{appname="test_track_and_gather",endpoint="default",sink_to="bigquery"} 3
// http_req_count{appname="test_track_and_gather",endpoint="matrix",sink_to="bigquery"} 1
// "#
//         .to_string();
//         assert_eq!(expected, res);
//
//         // counter metrics should be reset
//         let res = x.handle_gather_metrics();
//         assert_eq!("", res);
//
//         x._de_register_vecs();
//     }
//
//     // Purpose of this test is to make sure you don't have any name conflicts!
//     // can be moved to client side
//     #[test]
//     fn test_name_conflicts() {
//         use std::collections::HashSet;
//
//         let x = StatsdCollector::new(
//             "test_name_conflicts".to_string(),
//             vec![
//                 RegisterMetricInput {
//                     metric_type: MetricType::Counter,
//                     metric_name: "a1".to_string(),
//                     metric_desc: "Number of HTTP requests made.".to_string(),
//                     labels: vec![],
//                 },
//                 RegisterMetricInput {
//                     metric_type: MetricType::Histogram,
//                     metric_name: "a2".to_string(),
//                     metric_desc: "Number of HTTP requests made.".to_string(),
//                     labels: vec![],
//                 },
//             ],
//         );
//
//         let mut names = HashSet::new();
//         for name in x.counter_vec_map.keys() {
//             names.insert(name);
//         }
//         for name in x.histogram_vec_map.keys() {
//             names.insert(name);
//         }
//         for name in x.gauge_vec_map.keys() {
//             names.insert(name);
//         }
//         assert_eq!(
//             names.len(),
//             x.counter_vec_map.keys().len()
//                 + x.histogram_vec_map.keys().len()
//                 + x.gauge_vec_map.keys().len()
//         );
//         x._de_register_vecs();
//     }
// }
