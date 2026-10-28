use actix_web::web::Query;
use nbroutes_util::{def::*, timestamp};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

fn check_input<T: DeserializeOwned + Serialize>(body: Value, query: &str) {
    let before = timestamp();
    let mut now_body = body.clone();
    now_body["departure_time"] = json!("now");
    let input: T = serde_json::from_value(now_body).unwrap();
    let seconds = serde_json::to_value(input).unwrap()["departure_time"]
        .as_i64()
        .unwrap();
    assert!(seconds >= before && seconds <= timestamp());

    let input = Query::<T>::from_query(&format!("{}&departure_time=n%6Fw", query)).unwrap();
    let seconds = serde_json::to_value(input.into_inner()).unwrap()["departure_time"]
        .as_i64()
        .unwrap();
    assert!(seconds >= before && seconds <= timestamp());

    for value in [
        Value::Null,
        json!(0),
        json!(1793000037_i64),
        json!("1793000037"),
    ] {
        let mut input = body.clone();
        input["departure_time"] = value.clone();
        let input: T = serde_json::from_value(input).unwrap();
        let expected = if value.is_string() {
            json!(1793000037_i64)
        } else {
            value
        };
        assert_eq!(
            serde_json::to_value(input).unwrap()["departure_time"],
            expected
        );
    }
    assert!(
        serde_json::to_value(serde_json::from_value::<T>(body.clone()).unwrap()).unwrap()
            ["departure_time"]
            .is_null()
    );
    assert!(
        serde_json::to_value(Query::<T>::from_query(query).unwrap().into_inner()).unwrap()
            ["departure_time"]
            .is_null()
    );
    for value in ["0", "1793000037", "-1"] {
        let input = Query::<T>::from_query(&format!("{}&departure_time={}", query, value)).unwrap();
        assert_eq!(
            serde_json::to_value(input.into_inner()).unwrap()["departure_time"],
            value.parse::<i64>().unwrap()
        );
    }
    for value in [
        json!("NOW"),
        json!("invalid"),
        json!(""),
        json!(true),
        json!(1.5),
        json!(9223372036854775808_u64),
    ] {
        let mut input = body.clone();
        input["departure_time"] = value;
        assert!(serde_json::from_value::<T>(input).is_err());
    }
    for value in [
        "NOW",
        "invalid",
        "",
        "1.5",
        "9223372036854775808",
        "now&departure_time=1",
    ] {
        assert!(Query::<T>::from_query(&format!("{}&departure_time={}", query, value)).is_err());
    }
}

#[test]
fn all_departure_time_fields_accept_now_and_keep_numeric_types() {
    let directions = json!({"origin":"1,2","destination":"3,4"});
    let matrix = json!({"origins":"1,2","destinations":"3,4"});
    check_input::<DirectionsInput>(directions.clone(), "origin=1,2&destination=3,4");
    check_input::<ValhallaDirectionsInput>(directions, "origin=1,2&destination=3,4");
    check_input::<MatrixInput>(matrix.clone(), "origins=1,2&destinations=3,4");
    check_input::<ValhallaMatrixInput>(matrix, "origins=1,2&destinations=3,4");
    check_input::<ISOChroneValhallaInput>(json!({"coordinates":"1,2"}), "coordinates=1,2");
    check_input::<IsochroneInput>(json!({"center":"1,2"}), "center=1,2");
}

#[test]
fn async_matrix_http_uses_the_original_request_type() {
    actix_web::rt::System::new("async-matrix-now").block_on(async {
        use actix_web::{test, web, App, HttpResponse};
        async fn endpoint(input: web::Json<MassiveDistanceMatrixInput>) -> HttpResponse {
            HttpResponse::Ok().json(input.matrix_input.departure_time)
        }
        let mut app =
            test::init_service(App::new().route("/async_matrix", web::post().to(endpoint))).await;
        for value in [
            json!("now"),
            json!(1793000037_i64),
            Value::Null,
            json!("invalid"),
        ] {
            let before = timestamp();
            let request = test::TestRequest::post()
                .uri("/async_matrix")
                .set_json(&json!({
                    "task_id":"task","chunk_id":"chunk",
                    "matrix_input":{"origins":"1,2","destinations":"3,4","departure_time":value}
                }))
                .to_request();
            let response = test::call_service(&mut app, request).await;
            if value == json!("invalid") {
                assert_eq!(response.status(), 400);
            } else {
                assert_eq!(response.status(), 200);
                let output: Value =
                    serde_json::from_slice(&test::read_body(response).await).unwrap();
                if value == json!("now") {
                    assert!(
                        output.as_i64().unwrap() >= before
                            && output.as_i64().unwrap() <= timestamp()
                    );
                } else {
                    assert_eq!(output, value);
                }
            }
        }
    });
}
