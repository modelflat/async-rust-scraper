use scraper::{Html, Selector};
use crate::scraper::model::PokemonProduct;

/// Parses the HTML content and extracts a list of Pokemon products.
///
/// # Arguments
///
/// * `html` - The HTML content to parse.
///
/// # Returns
///
/// A list of `PokemonProduct` items extracted from the HTML.
pub fn parse_html(html: &str) -> Vec<PokemonProduct> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("li.product").unwrap();

    let mut data = Vec::new();
    for element in document.select(&selector) {
        let url = element
            .select(&Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);
        let image = element
            .select(&scraper::Selector::parse("img").unwrap())
            .next()
            .and_then(|img| img.value().attr("src"))
            .map(str::to_owned);
        let name = element
            .select(&scraper::Selector::parse("h2").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());
        let price = element
            .select(&scraper::Selector::parse(".price").unwrap())
            .next()
            .map(|price| price.text().collect::<String>());

        let pokemon_product = PokemonProduct {
            url,
            image,
            name,
            price,
        };
        data.push(pokemon_product);
    }
    data
}


