#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use crate::controllers::inventory::get_inventory;

mod entities;
mod dtos;
mod controllers;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![
            get_inventory
        ])
}