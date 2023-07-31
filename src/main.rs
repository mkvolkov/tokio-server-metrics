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

    let addr = ([127,0,0,1], 8081).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(response))});

    let server = Server::bind(&addr).serve(service);

    print!("Listening on http://{}\n", addr);

    server.await?;

    refr_handle.await.unwrap();

    Ok(())
}
