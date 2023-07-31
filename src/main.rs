use config::{Config, File};

mod cfg;
use cfg::Cfg;

mod measure;
use measure::refresh;

mod storage;

struct SiteRes {
    index: usize,
    time: String,
}

#[tokio::main]
async fn main() {
    let cfg = Config::builder()
        .add_source(File::with_name("cfg.yml"))
        .build()
        .unwrap();

    let parsed: Cfg = cfg.try_deserialize().unwrap();

    let host = parsed.RedisCfg.host.clone();
    let timeout = parsed.Timeout;
    let delay = parsed.RefreshTimeout;

    let refr_handle = tokio::spawn(refresh(host, timeout, delay));

    refr_handle.await.unwrap();
}