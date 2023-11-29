use octocrab::{models::repos::Release, Octocrab};
use std::error::Error;

#[derive(Clone, Debug)]
pub struct GitHub {
    client: Octocrab,
}

impl Default for GitHub {
    fn default() -> Self {
        Self::new()
    }
}

impl GitHub {
    pub fn new() -> Self {
        Self {
            client: Octocrab::builder()
                .add_header(
                    "User-Agent".parse().unwrap(),
                    "Rustina Assistant (rust@maid.uz)".to_string(),
                )
                .add_header(
                    "Authorization".parse().unwrap(),
                    std::env::var("GITHUB_TOKEN").unwrap(),
                )
                .build()
                .unwrap(),
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
