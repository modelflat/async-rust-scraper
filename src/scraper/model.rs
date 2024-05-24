/// This module contains the data models for the application.
///
/// Represents a Pokemon product scraped from the web.
#[derive(Debug)]
pub struct PokemonProduct {
    pub url: Option<String>,
    pub image: Option<String>,
    pub name: Option<String>,
    pub price: Option<String>,
}
