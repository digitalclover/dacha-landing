use warp::{http::Response, reject::Rejection, reply::Reply, Filter};

#[derive(serde::Deserialize, Debug)]
struct FormData {
    email: String,
    name: String,
}

pub fn register() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("register")
        .and(warp::post())
        .and(warp::header("user-agent"))
        .and(warp::body::form())
        .map(|agent: String, form_data: FormData| {
            println!("User-Agent:{}", agent);
            println!("Received data: {:?}", form_data);
            Response::builder().status(200).body("")
        })
}
