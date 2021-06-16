
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    use futures_util::stream::StreamExt;
    const MAX_CONCURRENT_JUMPERS: usize = 100;

    onvif::discovery::discover(std::time::Duration::from_secs(2))
        .await
        .unwrap()
        .for_each_concurrent(MAX_CONCURRENT_JUMPERS, |addr| async move {
            println!("Device found: {:?}", addr);
        })
        .await;
}
