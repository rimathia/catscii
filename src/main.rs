use serde::Deserialize;

#[derive(Deserialize)]
struct CatImage {
    url: String,
}

async fn get_cat_ascii_art() -> color_eyre::Result<String> {
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

    let image_bytes = client
        .get(image.url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    let image = image::load_from_memory(&image_bytes)?;
    let ascii_art = artem::convert(image, artem::options::OptionBuilder::new().build());

    Ok(ascii_art)
}

#[tokio::main]
async fn main() {
    let image = get_cat_ascii_art().await.unwrap();
    println!("{image}");
}
