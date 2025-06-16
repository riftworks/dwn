#![feature(thread_sleep_until)]
#[macro_use] extern crate rocket;

mod routes;

use routes::echo::echo;

#[rocket::main]
async fn main() -> () {
    let _ = rocket::build()
        .mount("/", routes![echo])
        .launch()
        .await;
}
