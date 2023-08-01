use serde::Deserialize;
use std::fs::read_to_string;

pub struct SiteTime {
    pub site: String,
    pub time: String,
}

#[derive(Debug, Deserialize)]
pub struct Cfg {
    pub redis_host: String,
    pub timeout: u64,
    pub refresh_timeout: u64,
    pub server_port: u16,
}

pub fn read_site_list() -> Vec<SiteTime> {
    let mut v = Vec::new();

    for line in read_to_string("sitelist.txt").unwrap().lines() {
        v.push(SiteTime { 
            site: line.to_string(),
            time: "unavailable".to_string(),
        })
    }

    v
}