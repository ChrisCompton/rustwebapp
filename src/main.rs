extern crate iron;
extern crate persistent;
extern crate router;
extern crate postgres;

use std::env;
use std::net::*;
use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use router::{Router};
use persistent::Write;
use postgres::{Connection, SslMode};

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
    println!("connecting to postgres");
    let conn = Connection::connect("postgres://dbuser:dbpass@172.17.0.18:5432/test", &SslMode::None).unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS messages (id INT PRIMARY KEY);", &[]).unwrap();

    conn.execute("INSERT INTO messages VALUES (1);", &[]).unwrap();
    conn.execute("INSERT INTO messages VALUES (2);", &[]).unwrap();
    conn.execute("INSERT INTO messages VALUES (3);", &[]).unwrap();

    let stmt = conn.prepare("SELECT id FROM messages;").unwrap();
    for row in stmt.query(&[]).unwrap() {
        let id: i32 = row.get(0);
        println!("id: {}", id);
    }


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