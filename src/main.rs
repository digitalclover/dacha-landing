use dacha_landing::run;

#[tokio::main]
async fn main() {
    let (server, addr, _tx) = run();
    println!("Serving at http://{}", addr);
    server.await;
}
