use octocrab::{models::repos::Release, Octocrab};
use std::error::Error;

#[derive(Clone, Debug)]
pub struct GitHub {
    client: Octocrab,
}

impl GitHub {
    pub fn new(token: Option<String>) -> Self {
        let client = Octocrab::builder();

        let client = client.add_header(
            "User-Agent".parse().unwrap(),
            "Rustacean Assistant (support@floss.uz)".to_string(),
        );

        match token {
            Some(t) => Self {
                client: client.personal_token(t.trim()).build().unwrap(),
            },
            None => Self {
                client: client.build().unwrap(),
            },
        }
    }

    pub async fn get_latest(&self) -> Result<Release, Box<dyn Error + Send + Sync>> {
        let latest = self
            .client
            .repos("rust-lang", "rust")
            .releases()
            .get_latest()
            .await?;

        Ok(latest)
    }

    pub async fn get_list(&self, page: u32) -> Result<Vec<Release>, Box<dyn Error + Send + Sync>> {
        let versions = self
            .client
            .repos("rust-lang", "rust")
            .releases()
            .list()
            .per_page(5)
            .page(page)
            .send()
            .await?
            .take_items();

        Ok(versions)
    }

    pub async fn get_detail(
        &self,
        tag_name: &str,
    ) -> Result<Release, Box<dyn Error + Send + Sync>> {
        let detail = self
            .client
            .repos("rust-lang", "rust")
            .releases()
            .get_by_tag(tag_name)
            .await?;

        Ok(detail)
    }
}
