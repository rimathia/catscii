use color_eyre::owo_colors::OwoColorize;
use serde::Deserialize;

#[derive(Deserialize)]
struct CatImage {
    id: String,
    url: String,
    width: usize,
    height: usize,
}

async fn get_cat_image_url() -> color_eyre::Result<String> {
    let api_url = "https://api.thecatapi.com/v1/images/search";
    let res = reqwest::get(api_url).await?;
    if !res.status().is_success() {
        return Err(color_eyre::eyre::eyre!(
            "api returned status {}",
            res.status()
        ));
    }
    let images: Vec<CatImage> = res.json().await?;
    let Some(image) = images.into_iter().next() else {
        return Err(color_eyre::eyre::eyre!("there is no image"));
    };
    Ok(image.url)
}

#[tokio::main]
async fn main() {
    let image_url = get_cat_image_url().await;
    println!("url: {}", image_url.unwrap());
}
