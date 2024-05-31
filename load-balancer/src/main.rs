use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0:0:0:80").await.unwrap();
    println!("Load balancer active on 0:0:0:80");
}
