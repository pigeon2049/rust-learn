use mysql::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref POOL: Pool = {
        // Access the environment variable at runtime
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL environment variable not set");

        // 将 String 转换为 &str 类型
        let database_url_ref: &str = &database_url;

        // 创建连接池
        Pool::new(database_url_ref)
            .expect("Failed to create connection pool")
    };
}