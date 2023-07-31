use std::fs::copy;

use config::{Config, File};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use redis::Client;

mod cfg;
use cfg::Cfg;

mod measure;
use measure::refresh;
use storage::get_val;

mod storage;
use storage::new_conn;

struct SiteRes {
    index: usize,
    time: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cfg = Config::builder()
        .add_source(File::with_name("cfg.yml"))
        .build()
        .unwrap();

    let parsed: Cfg = cfg.try_deserialize().unwrap();

    let host = parsed.RedisCfg.host.clone();
    let timeout = parsed.Timeout;
    let delay = parsed.RefreshTimeout;

    let refr_handle = tokio::spawn(refresh(host, timeout, delay));
/* 
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let mut m_conn = client.get_connection().unwrap();
*/

    let addr = ([127,0,0,1], 8081).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(response))});

    let server = Server::bind(&addr).serve(service);

    print!("Listening on http://{}\n", addr);

    server.await?;

    refr_handle.await.unwrap();

    Ok(())
}

async fn response(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
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

        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
