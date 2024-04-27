use dacha_landing::run;

#[tokio::main]
async fn main() {
    let (addr, _tx, server) = run();
    println!("Serving at http://{}", addr);
    server.await;
}
