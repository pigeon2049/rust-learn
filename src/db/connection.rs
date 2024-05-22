use mysql::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref POOL: Pool = {
        let url = "mysql://dev:password_2024@localhost:3306/dev";
        Pool::new(url).expect("Failed to create connection pool")
    };
}