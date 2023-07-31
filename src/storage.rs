use redis::{Client, Connection, RedisResult};
use redis::Commands;

pub fn new_conn(host: String) -> RedisResult<Connection> {
    let addr: String = format!("redis://{}/", host);
    let client = Client::open(addr)?;

    let mut conn = client.get_connection()?;

    RedisResult::Ok(conn)
}

pub fn set_val(conn: &mut Connection, key: String, val: String) -> RedisResult<()> {
    conn.set(key, val)?;

    Ok(())
}

pub fn get_val(conn: &mut Connection, key: String) -> RedisResult<(String)> {
    let val: String = conn.get(key)?;

    Ok(val)
}