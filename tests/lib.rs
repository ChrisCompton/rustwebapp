 #[test]
pub fn test_it_works() {
    assert!(true == true);
}

#[test]
#[cfg(feature = "integration_tests")]
pub fn test_db() {
    let conn_string:String = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => "postgres://dbuser:dbpass@localhost:5432/test".to_string()
    };

    println!("connecting to postgres: {}", conn_string);
    let pool = setup_connection_pool(&conn_string, 6);
    let conn = pool.get().unwrap();

    println!("connected to postgres");
    conn.execute("SELECT 1;", &[]).unwrap();
}