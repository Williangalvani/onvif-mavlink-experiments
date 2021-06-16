use std::collections::HashSet;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    use futures_util::stream::StreamExt;
    const MAX_CONCURRENT_JUMPERS: usize = 100;
    //let mut cameras = HashSet::new();

    let cameras_vec: Vec<onvif::discovery::Device>  = onvif::discovery::discover(std::time::Duration::from_secs(2))
        .await
        .unwrap()
        .collect().await;
    //for camera in &cameras {
        println!("{:#?}", cameras_vec);

    let set: HashSet<onvif::discovery::Device> = cameras_vec.iter().cloned().collect();
    println!("{:#?}", set);

}
