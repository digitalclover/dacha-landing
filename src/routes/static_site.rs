use std::env;

use warp::{http::Uri, reject::Rejection, reply::Reply, Filter};

const CONTENT_CONTROL: &str = "max-age=604800,public";

pub fn static_site() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let assets = warp::fs::dir(get_site_folder())
        .with(warp::compression::gzip())
        .map(|reply| {
            warp::reply::with_header(
                reply,
                "Content-Control",
                format!("{},public", CONTENT_CONTROL),
            )
        });
    let index = handle_index();
    assets.or(index)
}

fn get_site_folder() -> &'static str {
    match env::var("IS_REMOTE").is_ok() {
        true => "/home/dacha-admin/public",
        false => "public",
    }
}

fn handle_index() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end().and(warp::header::optional::<String>("Accept-Language").map(
        |lang_header: Option<String>| {
            if lang_header.map_or(false, |lang| lang.contains("en")) {
                warp::redirect::temporary(Uri::from_static("/en.html"))
            } else {
                warp::redirect::temporary(Uri::from_static("/ja.html"))
            }
        },
    ))
}
