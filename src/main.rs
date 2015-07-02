extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {

	println!("runnning iron...");

    Iron::new(|_: &mut Request| {
    	println!("got request");
        Ok(Response::with((status::Ok, "Hello world!")))
    }).http("0.0.0.0:8080").unwrap();

}