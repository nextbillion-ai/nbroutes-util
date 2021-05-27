use std::str::FromStr;
use std::string::ToString;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
