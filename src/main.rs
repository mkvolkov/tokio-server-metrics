use config::{Config, File};
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

mod cfg;
use cfg::Cfg;

mod handler;
use handler::response;

mod measure;
use measure::refresh;

mod storage;
mod metrics;

struct SiteRes {
    index: usize,
    time: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let raw_cfg = Config::builder()
        .add_source(File::with_name("cfg.yml"))
        .build()
        .unwrap();

    let cfg: Cfg = raw_cfg.try_deserialize().unwrap();

    let host = cfg.redis_host.to_string();
    let timeout = cfg.timeout;
    let delay = cfg.refresh_timeout;

    tokio::spawn(refresh(host, timeout, delay));

    let addr = ([127,0,0,1], cfg.server_port).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(response))});

    let server = Server::bind(&addr).serve(service);

    print!("Listening on http://{}\n", addr);

    server.await?;

    Ok(())
}
