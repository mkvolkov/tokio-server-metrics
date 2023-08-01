use serde::Deserialize;
use std::fs::read_to_string;

pub struct SiteTime {
    pub site: String,
    pub time: String,
}

#[derive(Debug, Deserialize)]
pub struct Cfg {
    pub RedisCfg: CfgRedis,
    pub Timeout: u64,
    pub RefreshTimeout: u64,
    RefreshStats: i32,
    FlushTime: i32,
    ServerCfg: Server,
}

#[derive(Debug, Deserialize)]
pub struct CfgRedis {
    pub host: String,
    pub port: String,
    pub password: String,
    maxidle: i32,
    maxconn: i32,
    sectime: i32,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    host: String,
    port: String,
}

pub fn ReadSiteList() -> Vec<SiteTime> {
    let mut v = Vec::new();

    for line in read_to_string("sitelist.txt").unwrap().lines() {
        v.push(SiteTime { 
            site: line.to_string(),
            time: "unavailable".to_string(),
        })
    }

    v
}