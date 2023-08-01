use hyper::{Body, Method, Request, Response, StatusCode};
use redis::Client;
use crate::storage::get_val;
use prometheus::{Encoder, TextEncoder};
use crate::metrics::{
    HTTP_USER_COUNTER, 
    HTTP_FASTEST_COUNTER,
    HTTP_SLOWEST_COUNTER,
    HTTP_SITE_COUNTER
};
use hyper::header::CONTENT_TYPE;

pub async fn response(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/st_site") => {
            HTTP_USER_COUNTER.inc();
            HTTP_SITE_COUNTER.inc();
            let bytes_key = hyper::body::to_bytes(req.into_body()).await?;
            let str_key = String::from_utf8(bytes_key.to_vec()).unwrap();

            let client = Client::open("redis://127.0.0.1/").unwrap();
            let mut m_conn = client.get_connection().unwrap();

            let res = get_val(&mut m_conn, str_key).unwrap();

            let body = format!("{}", res);

            Ok(Response::new(body.into()))
        }

        (&Method::GET, "/fastest") => {
            HTTP_USER_COUNTER.inc();
            HTTP_FASTEST_COUNTER.inc();
            let client = Client::open("redis://127.0.0.1/").unwrap();
            let mut m_conn = client.get_connection().unwrap();

            let res = get_val(&mut m_conn, "fastest".to_string()).unwrap();

            let body = format!("{}", res);

            Ok(Response::new(body.into()))
        }

        (&Method::GET, "/slowest") => {
            HTTP_USER_COUNTER.inc();
            HTTP_SLOWEST_COUNTER.inc();
            let client = Client::open("redis://127.0.0.1/").unwrap();
            let mut m_conn = client.get_connection().unwrap();

            let res = get_val(&mut m_conn, "slowest".to_string()).unwrap();

            let body = format!("{}", res);

            Ok(Response::new(body.into()))
        }

        (&Method::GET, "/admin/all") => {
            let encoder = TextEncoder::new();
        
            let metric_families = prometheus::gather();
            let mut buffer = vec![];
            encoder.encode(&metric_families, &mut buffer).unwrap();
        
            let response = Response::builder()
                .status(200)
                .header(CONTENT_TYPE, encoder.format_type())
                .body(Body::from(buffer))
                .unwrap();
        
            Ok(response)
        }

        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}