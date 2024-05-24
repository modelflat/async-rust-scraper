use reqwest::Error;
use crate::scraper::parser::parse_html;
use crate::scraper::model::PokemonProduct;

/// Fetches data from the given URL and parses the HTML to extract Pokemon products.
///
/// # Arguments
///
/// * `url` - The URL to fetch the data from.
///
/// # Returns
///
/// A Result containing a list of `PokemonProduct` items if successful, or an error if not.
pub async fn fetch_data(url: &str) -> Result<Vec<PokemonProduct>, Error> {
    log::info!("Fetching data from {}", url);
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let products = parse_html(&body);
    log::info!("Fetched {} products from {}", products.len(), url);
    Ok(products)
}
