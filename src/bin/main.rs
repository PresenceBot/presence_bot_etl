extern crate presence_bot;
extern crate diesel;

use presence_bot::*;
use self::models::*;
use diesel::prelude::*;

fn main() {
    use self::schema::Event::dsl::*;
    println!("Hello, world!");

    let connection = staging_connection();
    let results = Event
        .limit(5)
        .filter(name.eq("PRESENCE_UPDATE"))
        .load::<EventRecord>(&connection)
        .expect("error loading events");
    
    for event in results {
        println!("{:?}", event)
    }
}
