use moka::future::Cache;
use std::time::Duration;
use shared::Product;

pub fn build_products_cache() -> Cache<String, Vec<Product>> {
    Cache::builder().time_to_live(Duration::from_secs(30)).build()
}