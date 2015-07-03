extern crate iron;

use std::env;
use iron::prelude::*;
use iron::status;

fn main() {

    println!("runnning iron...");
    
    Iron::new(|_: &mut Request| {
        
        let powered_by:String = match env::var("POWERED_BY") {
            Ok(val) => val,
            Err(_) => "Deis".to_string()
        };
        println!("got request..");
        
        let message = format!("Powered by {}", powered_by);
        Ok(Response::with((status::Ok, message)))

    }).http("0.0.0.0:8080").unwrap();

}