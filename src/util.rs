use crate::def::{MaaasAreaConfig, MaaasConfig};
use crate::Result;
use async_process::Command;
use std::str::FromStr;
use std::string::ToString;

const EARTH_RADIUS_METER: f64 = 6373000.0_f64;

pub fn parse_list<T: FromStr>(input: &str) -> Result<Vec<T>> {
    let mut r: Vec<T> = Vec::new();
    let items = input.split("|");
    for item in items {
        match item.parse::<T>() {
            Ok(v) => {
                r.push(v);
            }
            Err(_) => bail!("invalid input"),
        }
    }
    Ok(r)
}

pub fn encode_list<T: ToString>(input: Vec<T>) -> String {
    let x: Vec<String> = input.iter().map(|x| x.to_string()).collect();
    x.join("|")
}

pub async fn gsutil(input: &str) -> Result<String> {
    let output = Command::new("gsutil").arg("cat").arg(input).output().await;
    if output.is_err() {
        warn!("error cat {:?} using gsutil: {:?}", input, output.err());
        bail!("error loading file using gsutil");
    }
    let output = output.unwrap();
    Ok(std::str::from_utf8(&output.stdout)?.to_owned())
}

pub async fn load_maaas_config() -> Result<MaaasConfig> {
    Ok(serde_yaml::from_str(
        &gsutil("gs://maaas/maaas-cfg.yaml").await?,
    )?)
}

pub async fn load_maaas_area_config() -> Result<MaaasAreaConfig> {
    Ok(serde_yaml::from_str(
        &gsutil("gs://maaas/maaas-area-cfg.yaml").await?,
    )?)
}

pub(crate) fn straight_distance(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    let start_latitude = lat1.to_radians();
    let end_latitude = lat2.to_radians();

    let delta_latitude = (lat1 - lat2).to_radians();
    let delta_longitude = (lng1 - lng2).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
        + start_latitude.cos() * end_latitude.cos() * (delta_longitude / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    (EARTH_RADIUS_METER * central_angle) as f64
}

//uncomment following testcase to ensure gsutil function works as expected
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_gsutil() {
        let r = gsutil("gs://saas-platform/maaas-cfg.yaml").await;
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.contains("areas"));
    }
}
*/
