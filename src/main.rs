use color_eyre::owo_colors::OwoColorize;
use pretty_hex::PrettyHex;
use serde::Deserialize;

#[derive(Deserialize)]
struct CatImage {
    url: String,
}

async fn get_cat_image_bytes() -> color_eyre::Result<Vec<u8>> {
    // let url = get_cat_image_url.await?;
    let client = reqwest::Client::default();
    let api_url = "https://api.thecatapi.com/v1/images/search";
    let image = client
        .get(api_url)
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<CatImage>>()
        .await?
        .pop()
        .ok_or_else(|| color_eyre::eyre::eyre!("no images returned"))?;

    Ok(client
        .get(image.url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?
        .to_vec())
}

#[tokio::main]
async fn main() {
    let image_bytes = get_cat_image_bytes().await.unwrap();
    // only dump the first 200 bytes so our terminal survives the
    // onslaught. this will panic if the image has fewer than 200 bytes.
    println!("{:?}", &image_bytes[..200].hex_dump());
}
