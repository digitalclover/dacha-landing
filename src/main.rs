use dacha_landing::startup::run;

#[tokio::main]
async fn main() {
    let (addr, _tx, server) = run();
    println!("Serving at http://{}", addr);
    server.await;
}
