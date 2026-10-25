#[derive(Debug, Default)]
pub struct TransferLearner {
    pub source_domains: Vec<String>,
}

impl TransferLearner {
    pub fn register_domain(&mut self, domain: impl Into<String>) {
        let domain = domain.into();
        if !self.source_domains.contains(&domain) { self.source_domains.push(domain); }
    }

    pub fn known(&self, domain: &str) -> bool { self.source_domains.iter().any(|d| d == domain) }
}
