use warp::{Filter, Future};

pub fn run() -> impl Future<Output = ()> {
    let hello = warp::path("health_check").map(warp::reply);
    warp::serve(hello).run(([127, 0, 0, 1], 8000))
}
