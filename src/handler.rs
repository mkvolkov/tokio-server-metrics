use hyper::{Body, Method, Request, Response, StatusCode};
use redis::Client;
use crate::storage::get_val;

pub async fn response(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/st_site") => {
            let bytes_key = hyper::body::to_bytes(req.into_body()).await?;
            let str_key = String::from_utf8(bytes_key.to_vec()).unwrap();

            let client = Client::open("redis://127.0.0.1/").unwrap();
            let mut m_conn = client.get_connection().unwrap();

            let res = get_val(&mut m_conn, str_key).unwrap();

            let body = format!("{}", res);

            Ok(Response::new(body.into()))
        }

        (&Method::GET, "/fastest") => {
            let client = Client::open("redis://127.0.0.1/").unwrap();
            let mut m_conn = client.get_connection().unwrap();

            let res = get_val(&mut m_conn, "fastest".to_string()).unwrap();

            let body = format!("{}", res);

            Ok(Response::new(body.into()))
        }

        (&Method::GET, "/slowest") => {
            let client = Client::open("redis://127.0.0.1/").unwrap();
            let mut m_conn = client.get_connection().unwrap();

            let res = get_val(&mut m_conn, "slowest".to_string()).unwrap();

            let body = format!("{}", res);

            Ok(Response::new(body.into()))
        }

        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}