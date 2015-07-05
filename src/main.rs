extern crate iron;
extern crate persistent;
extern crate router;

use std::env;
use std::net::*;
use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use router::{Router};
use persistent::Write;

#[derive(Copy, Clone)]
pub struct HitCounter;

impl Key for HitCounter { type Value = usize; }

fn index(_: &mut Request) -> IronResult<Response> {
    let powered_by:String = match env::var("POWERED_BY") {
        Ok(val) => val,
        Err(_) => "Iron".to_string()
    };
    let message = format!("Powered by: {}", powered_by);
    Ok(Response::with((status::Ok, message)))
}

fn posts(req: &mut Request) -> IronResult<Response> {
    let ref post_id = req.extensions.get::<Router>().unwrap().find("post_id").unwrap_or("none");
    Ok(Response::with((status::Ok, "PostId: {}", *post_id)))
}

fn hits(req: &mut Request) -> IronResult<Response> {
    let mutex = req.get::<Write<HitCounter>>().unwrap();
    let mut count = mutex.lock().unwrap();
    *count += 1;
    Ok(Response::with((status::Ok, format!("Hits: {}", *count))))
}

fn main() {    
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let port = 8080;
    let address = SocketAddrV4::new(ip, port);
    
    println!("listening on http://{}", address);

    let mut router = Router::new();

    router.get("/", index);
    router.get("/posts/:post_id", posts);
    router.get("/hits", hits);

    let mut middleware = Chain::new(router);
    middleware.link(Write::<HitCounter>::both(0));

    Iron::new(middleware).http(address).unwrap();
}