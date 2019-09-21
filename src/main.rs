use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::io;

#[derive(Deserialize, Debug)]
struct Record {
    _id: String,
    greeting: String,
}

type Records = Vec<Record>;

const URL: &str = "https://jsonbox.io/box_ed82aef3f93176996147";

fn main() -> Result<(), Box<dyn Error>> {
    let records: Records = reqwest::get(URL)?.json()?;
    if let Some(record) = records.first() {
        println!("Greeting from previous guest: {}", record.greeting.trim());
    } else {
        println!("No greeting, you're the first.");
    }

    println!("Please input greeting for next guest.");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let mut payload = HashMap::new();
    payload.insert("greeting", buffer.trim());

    let client = reqwest::Client::new();
    let mut res = client.post(URL).json(&payload).send()?;
    println!("{:?}", res.text());
    Ok(())
}
