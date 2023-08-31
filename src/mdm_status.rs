use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::def::{MassiveDistanceMatrixStatus, MassiveDistanceMatrixStatusEnum};
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    static ref STATUS: Arc<Mutex<HashMap<String, MassiveDistanceMatrixStatus>>> = Arc::new(Mutex::new(HashMap::new()));
}

const EXPIRA_TIME_24H: i64 = 24 * 60 * 60 * 1000; // 12h
// const EXPIRA_TIME_5S: i64 = 10 * 1000; // 10s

pub fn get_status(task_id: String) -> MassiveDistanceMatrixStatus {
    // run in mdm mode no need evict, becase pod will be release after used
    // evict();
    
    let m = STATUS.lock().unwrap();
    let status = m.get(&task_id).clone();
    if status.is_some(){
        return status.unwrap().clone()
    }

    return MassiveDistanceMatrixStatus{
        task_id,
        status: MassiveDistanceMatrixStatusEnum::NoExist,
        message: "".to_string(),
        start_time: 0,
        output: None,
    }
}

pub fn set_status(task_id: String, status: MassiveDistanceMatrixStatus)  {
    STATUS.lock().unwrap().insert(task_id, status);
    return
}

pub fn evict(){
    let now_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    let mut delete_list: Vec<String> = Vec::new();
    for (key, value) in STATUS.lock().unwrap().iter() {
        if now_time - value.start_time > EXPIRA_TIME_24H{
            delete_list.push(key.to_string())
        }
    }

    for key in delete_list{
        STATUS.lock().unwrap().remove(&key);
    }
}


