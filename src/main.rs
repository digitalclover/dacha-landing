use dacha_landing::run;

#[tokio::main]
async fn main() {
    let (addr, server) = run().unwrap();
    println!("Serving at http://{}", addr);
    server.await;
}
