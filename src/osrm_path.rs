pub fn get_data_root() -> String {
    std::env::var("DATA_PATH").unwrap_or("/osrm".to_string())
}
