use std::net::SocketAddr;
use warp::{Error, Filter, Future};

pub fn run() -> Result<(SocketAddr, impl Future<Output = ()> + 'static), Error> {
    let hello = warp::path("health_check").map(warp::reply);
    warp::serve(hello).try_bind_ephemeral(([127, 0, 0, 1], 0))
}
