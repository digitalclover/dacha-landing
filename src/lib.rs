use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::sync::oneshot::{self, Sender};
use warp::{http::header::HeaderValue, Filter, Future, Rejection, Reply};

const EN_TARGET: &str = "en";
const CONTENT_CONTROL: &str = "max-age=604800,public";

fn get_site_folder() -> &'static str {
    match env::var("IS_REMOTE").is_ok() {
        true => "/home/dacha-admin/public",
        false => "public",
    }
}

fn get_index() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end().and(
        warp::header::value("Accept-Language")
            .map(|value: HeaderValue| match value.to_str() {
                Ok(lang) if (lang.contains(EN_TARGET)) => "en",
                _ => "ja",
            })
            .map(|lang| {
                let path = format!("/{}.html", lang)
                    .parse::<warp::http::Uri>()
                    .unwrap();
                warp::reply::with_header(warp::redirect::see_other(path), "Content-Language", lang)
            })
            .map(|reply| {
                warp::reply::with_header(
                    reply,
                    "Content-Control",
                    format!("{},public", CONTENT_CONTROL),
                )
            }),
    )
}

pub fn run() -> (SocketAddr, Sender<()>, impl Future<Output = ()> + 'static) {
    let health_check = warp::path("health_check").and(warp::get()).map(warp::reply);

    let assets = warp::fs::dir(get_site_folder())
        .with(warp::compression::gzip())
        .map(|reply| {
            warp::reply::with_header(
                reply,
                "Content-Control",
                format!("{},public", CONTENT_CONTROL),
            )
        });
    let index = get_index();
    let site = assets.or(index);
    let routes = health_check.or(site);

    let (tx, rx) = oneshot::channel::<()>();
    let (local_ip, port) = get_local_ip();

    let target_addr = SocketAddr::new(IpAddr::V4(local_ip), port);
    let (addr, server) = warp::serve(routes).bind_with_graceful_shutdown(target_addr, async {
        rx.await.ok();
    });

    (addr, tx, server)
}

fn get_local_ip() -> (Ipv4Addr, u16) {
    let is_remote = env::var("IS_REMOTE").is_ok();
    match is_remote {
        true => (Ipv4Addr::new(10, 0, 0, 4), 8080),
        false => (Ipv4Addr::new(127, 0, 0, 1), 0),
    }
}
