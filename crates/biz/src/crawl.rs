use entity::asn;
use pkgs::Error;
use std::sync::Arc;

#[async_trait::async_trait(?Send)]
pub trait CrawlRepo {
    async fn crawl(&self, url: &str) -> Result<Vec<asn::Model>, Error>;
}

pub struct CrawlUsecase {
    repo: Arc<dyn CrawlRepo>,
}

unsafe impl Send for CrawlUsecase {}
unsafe impl Sync for CrawlUsecase {}

impl CrawlUsecase {
    pub fn new(repo: Arc<dyn CrawlRepo>) -> Self {
        Self { repo }
    }

    pub async fn crawl(&self, url: &str) -> Result<Vec<asn::Model>, Error> {
        self.repo.crawl(url).await
    }
}
