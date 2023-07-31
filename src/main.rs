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

    let refr_handle = tokio::spawn(refresh(parsed.r_host()));

    refr_handle.await.unwrap();
}