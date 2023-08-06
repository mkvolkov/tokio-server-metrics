# tokio-server-metrics

Asynchronous multithreaded server, based on tokio and hyper.
Measures the time to reach to several sites with timeout.
Gives the time to reach the site in milliseconds, the site
with the fastest access and with the slowest access.

Collects the number of requests.

Requirements:

- Redis (installed, launched)
- Prometheus (installed, launched)

URLs to call:

```bash
curl http://localhost:8081/fastest

curl http://localhost:8081/slowest

curl http://localhost:8081/st_site -XGET -d 'google.com'

curl http://localhost:8081/admin/all

curl http://localhost:8081/metrics
```

Prometheus configuration:

```
scrape_configs:

  - job_name: "tokio-server-metrics"
    scrape_interval: 10s
    static_configs:
      - targets: ["localhost:8081"]

```