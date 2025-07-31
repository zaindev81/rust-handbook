use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;

#[tokio::main]
async fn main() {
   let manager = RedisConnectionManager::new("redis://localhost").unwrap(); // create manager for connection
   let pool = bb8::Pool::builder().build(manager).await.unwrap(); // create pool

    {
        // ping the database before starting
        let mut conn = pool.get().await.unwrap();
        conn.set::<&str, &str, ()>("foo", "bar").await.unwrap();
        let result: String = conn.get("foo").await.unwrap();
        println!("result: {}", result);
        assert_eq!(result, "bar");
   }
}