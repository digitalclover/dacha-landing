use dacha_landing::run;

#[tokio::main]
async fn main() {
    let (addr, _tx, server) = run(80);
    println!("Serving at http://{}", addr);
    server.await;
}
