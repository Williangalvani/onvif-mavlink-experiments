use std::collections::HashSet;
use url::Url;

#[tokio::main]
async fn main() {

    use futures_util::stream::StreamExt;

    let cameras_vec: Vec<onvif::discovery::Device>  = onvif::discovery::discover(std::time::Duration::from_secs(2))
        .await
        .unwrap()
        .collect().await;
    println!("{:#?}", cameras_vec);

    let set: HashSet<Url> = cameras_vec.iter().map(|camera| camera.url.clone()).collect();
    println!("{:#?}", set);

}
