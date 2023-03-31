use std::{
    env,
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
    process::Command,
};

use axum::{
    extract::{ConnectInfo, State, Query},
    http::StatusCode,
    routing::get,
    Router,
};

use once_cell::sync::Lazy;

static IS_URL: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(r##"^https?://.*"##).expect("Failed to compile IS_URL regex")
});

const BAD_REQUEST: StatusCode = StatusCode::BAD_REQUEST;
const SERVER_ERROR: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
const UNAUTHORIZED: StatusCode = StatusCode::UNAUTHORIZED;
const OK: StatusCode = StatusCode::OK;

struct Config {
    listen_ip: Ipv4Addr,
    listen_port: u16,
    browser_cmd: String,
    restrict_ip: Option<Ipv4Addr>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listen_ip: "0.0.0.0".parse().unwrap(),
            listen_port: 5001,
            browser_cmd: String::from("open"),
            restrict_ip: None,
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut config = Config::default();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Usage:\n\n  listener <listen port> <browser command> [restrict to IP]\n\nPassing an IP address as the lasp parameter causes listener to reject any requests NOT from that IP.\n\nmacOS example:\n    listener 5000 open\n\nLinux example:\n    listener 6969 xdg-open 142.250.179.174");
    }

    config.listen_port = args[1].parse().expect("Invalid port number provided");
    config.browser_cmd = args[2].clone();
    config.restrict_ip = args.get(3).map(|a| {
        a.parse().unwrap_or_else(|_| panic!("Failed to parse {a} as restrict IP"))
    });
    let addr = SocketAddr::from((config.listen_ip, config.listen_port));
    let shared_state = Arc::new(config);

    let app = Router::new()
        .route("/", get(open_url))
        .with_state(shared_state.clone());

    log::info!("Listening on port {}", shared_state.listen_port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn open_url(
    Query(params): Query<HashMap<String, String>>,
    ConnectInfo(remote_addr): ConnectInfo<SocketAddr>,
    State(config): State<Arc<Config>>
) -> (StatusCode, String) {
    let url = match params.get("url") {
        Some(u) => u,
        None => return (BAD_REQUEST, String::from("No `url` parameter given")),
    };

    if !IS_URL.is_match(url) {
        return (BAD_REQUEST, format!("Invalid URL given: {url}"));
    }

    let remote_ip: IpAddr = remote_addr.ip();
    if config.restrict_ip.is_some() && remote_ip.is_ipv4() &&
        IpAddr::V4(config.restrict_ip.unwrap()) != remote_ip {
        return (UNAUTHORIZED, "You are not authorized to access this URI".to_string());
    }

    if let Err(e) = Command::new(&config.browser_cmd)
        .args([url])
        .output() {
            log::error!("Failed to open {url}: {:?}", e);
            return (SERVER_ERROR, format!("Failed to open {url} on remote machine"));
        }

    let _ = Command::new("notify-send")
        .args(["Listener opened a URL", &format!(r#"<a href="{url}">{url}</a>"#)])
        .output();

    log::info!("Opened URL from {remote_addr} → {url}");
    (OK, format!("Opened URL {url}"))
}
