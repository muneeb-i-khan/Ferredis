use std::collections::HashMap;
use tokio::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref TABLE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub async fn set(key: &String, value: &String) {
    let mut table = TABLE.lock().await;
    table.insert(key.clone(), value.clone());
}

pub async fn get(key: &String) -> Option<String> {
    let table = TABLE.lock().await;
    table.get(key).cloned()
}
