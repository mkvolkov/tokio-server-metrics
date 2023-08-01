use lazy_static::lazy_static;
use prometheus::{opts, register_counter};
use prometheus::Counter;

lazy_static! {
    pub static ref HTTP_USER_COUNTER: Counter = register_counter!(opts!(
        "example_http_requests_total",
        "Number of HTTP requests made by users.",
    ))
    .unwrap();
    pub static ref HTTP_FASTEST_COUNTER: Counter = register_counter!(opts!(
        "example_http_requests_fastest",
        "Number of HTTP requests /fastest.",
    ))
    .unwrap();
    pub static ref HTTP_SLOWEST_COUNTER: Counter = register_counter!(opts!(
        "example_http_requests_slowest",
        "Number of HTTP requests /slowest.",
    ))
    .unwrap();
    pub static ref HTTP_SITE_COUNTER: Counter = register_counter!(opts!(
        "example_http_requests_st_site",
        "Number of HTTP requests /st_site.",
    ))
    .unwrap();
}