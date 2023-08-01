use hyper::Client;
use std::time::{Duration, Instant};
use tokio::io::Result;
use tokio::time::sleep;

use crate::SiteRes;
use crate::cfg::SiteTime;
use crate::cfg::read_site_list;
use crate::storage::new_conn;
use crate::storage::set_val;

const THOUS: u64 = 1000;

pub async fn refresh(host: String, timeout: u64, delay: u64) {
    let mut v_main: Vec<SiteTime> = read_site_list();

    let n_sites = v_main.len();

    let mut r_conn = new_conn(host).unwrap();

    loop {
        

        let mut handles = vec![];
        for k in 0..n_sites {
            let site = v_main[k].site.clone();
            let handle = tokio::spawn(measure(k, timeout, site));
            handles.push(handle);
        }
    
        for handle in handles {
            let res = handle.await.unwrap();
            if let Ok(sres) = res {
                v_main[sres.index].time = sres.time;
            }
        }

        let mut fastest: u64 = timeout * THOUS;
        let mut f_site: String = "unknown".to_string();
        let mut slowest: u64 = 1;
        let mut s_site: String = "unknown".to_string();
        for k in 0..v_main.len() {
            let stime = v_main[k].time.parse::<u64>();
            match stime {
                Ok(st) => {
                    if st < fastest {
                        fastest = st;
                        f_site = v_main[k].site.clone();
                    }
                    if st > slowest {
                        slowest = st;
                        s_site = v_main[k].site.clone();
                    }
                }
                Err(_) => {
                    continue;
                }
            }
        }

        for k in 0..v_main.len() {
            set_val(& mut r_conn, v_main[k].site.clone(), v_main[k].time.clone()).unwrap();
        }

        set_val(& mut r_conn, "fastest".to_string(), f_site).unwrap();
        set_val(& mut r_conn, "slowest".to_string(), s_site).unwrap();

        sleep(Duration::from_secs(delay)).await;
    }
}

async fn measure(n: usize, timeout: u64, site: String) -> Result<SiteRes> {
    let uri = ("http://".to_string() + &site).parse().unwrap();
    let client = Client::new();
    let start = Instant::now();
    let work = client.get(uri);
    let mut elapsed_time: Duration = Duration::from_millis(0);

    match tokio::time::timeout(Duration::from_secs(timeout), work).await {
        Ok(res) => match res {
            Ok(_) => {
                elapsed_time = start.elapsed();
            }
            Err(_) => {},
        },
        Err(_) => {},
    };

    let time_res: String;

    if elapsed_time.as_millis() == 0 {
        time_res = "unavailable".to_string();
    } else {
        time_res = elapsed_time.as_millis().to_string();
    }

    Ok(SiteRes { 
        index: n, 
        time: time_res,
    })
}