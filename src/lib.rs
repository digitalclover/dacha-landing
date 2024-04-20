use std::net::SocketAddr;
use warp::{http::header::HeaderValue, Error, Filter, Future, Rejection, Reply};

const SITE_FOLDER: &str = "public";
const EN_TARGET: &str = "en";

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
}

pub fn run() -> Result<(SocketAddr, impl Future<Output = ()> + 'static), Error> {
    let health_check = warp::path("health_check").and(warp::get()).map(warp::reply);

    let assets = warp::fs::dir(SITE_FOLDER);
    let index = get_index();
    let site = assets.or(index);
    let routes = health_check.or(site);
    warp::serve(routes).try_bind_ephemeral(([127, 0, 0, 1], 0))
}
