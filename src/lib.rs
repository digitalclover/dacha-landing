use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::sync::oneshot::{self, Sender};
use warp::{http::header::HeaderValue, Filter, Future, Rejection, Reply};

const SITE_FOLDER: &str = "public";
const EN_TARGET: &str = "en";
const CONTENT_CONTROL: &str = "max-age=604800,public";

fn get_index() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
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
        })
}

pub fn run(port: u16) -> (SocketAddr, Sender<()>, impl Future<Output = ()> + 'static) {
    let health_check = warp::path("health_check").and(warp::get()).map(warp::reply);

    let assets = warp::fs::dir(SITE_FOLDER)
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

    let target_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let (addr, server) = warp::serve(routes).bind_with_graceful_shutdown(target_addr, async {
        rx.await.ok();
    });

    (addr, tx, server)
}
