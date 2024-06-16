use warp::{reject::Rejection, reply::Reply, Filter};

pub fn health_check() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("health_check").and(warp::get()).map(warp::reply)
}
