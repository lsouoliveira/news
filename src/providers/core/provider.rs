use crate::providers::core::data::{Article, Paginated};
use crate::providers::core::client::Client;
use crate::providers::core::error::Error;

pub trait Provider {
    fn new(client: Client) -> Self;
    async fn get_articles(&self) -> Result<Paginated<Article>, Error>;
}
