use std::net::SocketAddr;

use tokio::sync::oneshot::{self, Sender};
use warp::{Filter, Future};

pub fn run() -> (impl Future<Output = ()>, SocketAddr, Sender<()>) {
    let (tx, rx) = oneshot::channel();
    let hello = warp::path("health_check").map(warp::reply);
    let (addr, server) =
        warp::serve(hello).bind_with_graceful_shutdown(([127, 0, 0, 1], 0), async {
            rx.await.ok();
        });
    (server, addr, tx)
}
