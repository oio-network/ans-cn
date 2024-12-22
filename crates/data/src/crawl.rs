use biz::CrawlRepo as BizCrawlRepo;
use crawler::Crawler;
use entity::asn;
use pkgs::Error;
use std::sync::Arc;

pub struct CrawlRepo {
    crawler: Arc<Crawler>,
}

impl CrawlRepo {
    pub fn new(crawler: Arc<Crawler>) -> Self {
        Self { crawler }
    }
}

#[async_trait::async_trait(?Send)]
impl BizCrawlRepo for CrawlRepo {
    async fn crawl(&self, url: &str) -> Result<Vec<asn::Model>, Error> {
        Ok(self.crawler.asn(url).await?)
    }
}
