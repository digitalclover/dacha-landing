use warp::{http::Response, reject::Rejection, reply::Reply, Filter};

pub fn health_check() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("health_check")
        .and(warp::get())
        .and(warp::header("user-agent"))
        .map(|agent: String| {
            println!("User-Agent:{}", agent);
            Response::builder().status(200).body("")
        })
}
