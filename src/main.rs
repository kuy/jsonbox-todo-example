use jsonbox::client::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub title: String,
    pub done: bool,
}

fn main() -> Result<(), Error> {
    let client = Client::new("it_is_looks_like_jsonbox_example");

    let mut todo = Todo {
        title: "Remember the Milk!".into(),
        done: false,
    };
    let (record, meta) = client.create(&todo)?;
    println!("Created: {:?}", record);

    todo.done = true;
    client.update(&meta.id, &todo)?;
    let (record, _) = client.read().id::<Todo>(&meta.id)?;
    println!("Updated: {:?}", record);

    Ok(())
}
