extern crate iron;
extern crate persistent;
extern crate router;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

use std::env;
use std::net::*;
use std::sync::Arc;
use std::thread;
use std::default::Default;

use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use router::{Router};
use persistent::Write;

use postgres::{Connection, SslMode};
use r2d2::{Pool, PooledConnection, Config};
use r2d2_postgres::{PostgresConnectionManager};

macro_rules! try {
    ($e:expr) => (
        match $e {
            Ok(ok) => ok,
            Err(err) => panic!("{:#?}", err)
        }
    )
}

#[derive(Copy, Clone)]
pub struct HitCounter;
impl Key for HitCounter { type Value = usize; }

pub type PostgresPool = Pool<PostgresConnectionManager>;
pub type PostgresPooledConnection = PooledConnection<PostgresConnectionManager>;

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

pub fn setup(cn_str: &str, pool_size: u32) -> PostgresPool {
    let manager = ::r2d2_postgres::PostgresConnectionManager::new(cn_str, ::postgres::SslMode::None).unwrap();
    
    let config = ::r2d2::Config::builder()
        .pool_size(pool_size)
        .build();    

    ::r2d2::Pool::new(config, manager).unwrap()
}

fn main() {

    println!("connecting to postgres");

    //let config = r2d2::Config::default();
    //let manager:PostgresConnectionManager = PostgresConnectionManager::new("postgres://dbuser:dbpass@172.17.0.18:5432/test", SslMode::None).unwrap();
    //let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());

    let pool:PostgresPool = setup("postgres://dbuser:dbpass@172.17.0.18:5432/test", 6);
    let conn:PostgresPooledConnection = pool.get().unwrap();

    try!(conn.execute("DROP TABLE IF EXISTS messages;", &[]));
    try!(conn.execute("CREATE TABLE IF NOT EXISTS messages (id INT PRIMARY KEY);", &[]));

    try!(conn.execute("INSERT INTO messages VALUES (1);", &[]));
    try!(conn.execute("INSERT INTO messages VALUES (2);", &[]));
    try!(conn.execute("INSERT INTO messages VALUES (3);", &[]));

    let stmt = try!(conn.prepare("SELECT id FROM messages;"));
    for row in try!(stmt.query(&[])) {
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