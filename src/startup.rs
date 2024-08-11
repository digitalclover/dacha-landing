use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::sync::oneshot::{self, Sender};
use warp::{Filter, Future};

use crate::routes::{health_check, register, static_site};

pub fn run() -> (SocketAddr, Sender<()>, impl Future<Output = ()> + 'static) {
    let site = static_site();
    let health_check = health_check();
    let register = register();
    let routes = health_check.or(register).or(site);

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
