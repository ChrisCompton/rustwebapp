extern crate iron;
extern crate persistent;
extern crate router;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

use std::env;
use std::net::*;

use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use router::Router;
use persistent::{Write,Read};

use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager};

pub type PostgresPool = Pool<PostgresConnectionManager>;
pub type PostgresPooledConnection = PooledConnection<PostgresConnectionManager>;

#[derive(Copy, Clone)]
pub struct HitCounter;
impl Key for HitCounter { type Value = usize; }

pub struct AppDb;
impl Key for AppDb { type Value = PostgresPool; }


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

fn database(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<AppDb>>().unwrap();
    let conn = pool.get().unwrap();
    let stmt = conn.prepare("SELECT id FROM messages;").unwrap();
    for row in stmt.query(&[]).unwrap() {
        let id: i32 = row.get(0);
        println!("id: {}", id);
    }
    Ok(Response::with((status::Ok, format!("Db: {}", "ok"))))
}

pub fn setup_connection_pool(cn_str: &str, pool_size: u32) -> PostgresPool {
    let manager = ::r2d2_postgres::PostgresConnectionManager::new(cn_str, ::postgres::SslMode::None).unwrap();
    let config = ::r2d2::Config::builder().pool_size(pool_size).build();
    ::r2d2::Pool::new(config, manager).unwrap()
}

fn main() {
    let conn_string:String = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => "postgres://dbuser:dbpass@localhost:5432/test".to_string()
    };

    println!("connecting to postgres: {}", conn_string);

    let pool = setup_connection_pool(&conn_string, 6);
    let conn = pool.get().unwrap();

    println!("connected to postgres");

    conn.execute("DROP TABLE IF EXISTS messages;", &[]).unwrap();    
    conn.execute("CREATE TABLE IF NOT EXISTS messages (id INT PRIMARY KEY);", &[]).unwrap();
    
    conn.execute("INSERT INTO messages VALUES (1);", &[]).unwrap();
    conn.execute("INSERT INTO messages VALUES (2);", &[]).unwrap();
    conn.execute("INSERT INTO messages VALUES (3);", &[]).unwrap();
    
    let mut router = Router::new();
    router.get("/", index);
    router.get("/posts/:post_id", posts);
    router.get("/hits", hits);
    router.get("/database", database);

    let mut middleware = Chain::new(router);
    middleware.link(Write::<HitCounter>::both(0));
    middleware.link(Read::<AppDb>::both(pool));    

    let host = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080);
    println!("listening on http://{}", host);
    Iron::new(middleware).http(host).unwrap();
}

#[test]
pub fn test_it_works() {
    assert!(true == true);
}

fn main() {
    println!("Working!................");
}