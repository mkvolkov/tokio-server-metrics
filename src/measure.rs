use hyper::Client;
use std::time::{Duration, Instant};
use tokio::io::Result;

use crate::SiteRes;
use crate::cfg::SiteTime;
use crate::cfg::ReadSiteList;
use crate::storage::new_conn;
use crate::storage::set_val;

pub async fn refresh(host: String) {
    let mut v_main: Vec<SiteTime> = ReadSiteList();

    let n_sites = v_main.len();

    let mut r_conn = new_conn(host).unwrap();

    loop {
        let mut handles = vec![];
        for k in 0..n_sites {
            let site = v_main[k].site.clone();
            let handle = tokio::spawn(measure(k, site));
            handles.push(handle);
        }
    
        for handle in handles {
            let res = handle.await.unwrap();
            if let Ok(sres) = res {
                v_main[sres.index].time = sres.time;
            }
        }

        for k in 0..v_main.len() {
            set_val(& mut r_conn, v_main[k].site.clone(), v_main[k].time.clone()).unwrap();
        }
    }
}

async fn measure(n: usize, site: String) -> Result<SiteRes> {
    let uri = ("http://".to_string() + &site).parse().unwrap();
    let client = Client::new();
    let start = Instant::now();
    let work = client.get(uri);
    let mut elapsed_time: Duration = Duration::from_millis(0);

    match tokio::time::timeout(Duration::from_secs(10), work).await {
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