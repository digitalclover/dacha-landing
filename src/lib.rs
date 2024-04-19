use std::net::SocketAddr;
use warp::{Error, Filter, Future};

const SITE_FOLDER: &str = "public";

pub fn run() -> Result<(SocketAddr, impl Future<Output = ()> + 'static), Error> {
    let health_check = warp::path("health_check").and(warp::get()).map(warp::reply);

    let assets = warp::fs::dir(SITE_FOLDER);
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/en.html", SITE_FOLDER)));
    let site = assets.or(index);
    let routes = health_check.or(site);
    warp::serve(routes).try_bind_ephemeral(([127, 0, 0, 1], 0))
}
