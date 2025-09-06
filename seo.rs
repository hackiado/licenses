use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Seo {
    /// Title (~60 chars)
    pub title: String,
    /// Description (~160 chars)
    pub description: String,
    pub keywords: Vec<String>,
    pub author: Option<String>,

    pub canonical_url: Option<String>,
    pub lang: Option<String>,    // ex: "fr"
    pub updated: Option<String>, // ISO8601

    // Social
    pub og_image: Option<String>,
    pub og_type: Option<String>,      // "website" | "article" | "book"…
    pub twitter_card: Option<String>, // "summary_large_image"

    // (Optionnel) Pour générer du JSON-LD
    pub json_ld: Option<String>,

    // (Optionnel) Jardin
    pub content_type: Option<String>, // "work" | "author" | "season" | "event"
    pub slug: Option<String>,
}

impl Seo {
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            keywords: Vec::new(),
            author: None,
            canonical_url: None,
            lang: None,
            updated: None,
            og_image: None,
            og_type: None,
            twitter_card: None,
            json_ld: None,
            content_type: None,
            slug: None,
        }
    }
    pub fn with_title(&mut self, t: &str) -> &mut Self {
        self.title.clear();
        self.title.push_str(t);
        self
    }
    pub fn with_desc(&mut self, d: &str) -> &mut Self {
        self.description.clear();
        self.description.push_str(d);
        self
    }
    pub fn with_keywords(&mut self, ks: &[&str]) -> &mut Self {
        self.keywords.clear();
        self.keywords.extend(ks.iter().map(|s| s.to_string()));
        self
    }
    pub fn canonical(&mut self, url: &str) -> &mut Self {
        self.canonical_url.replace(url.to_string());
        self
    }
    pub fn og(&mut self, image: &str, kind: &str) -> &mut Self {
        self.og_image.replace(image.to_string());
        self.og_type.replace(kind.to_string());
        self
    }
    pub fn twitter_large(&mut self) -> &mut Self {
        self.twitter_card.replace("summary_large_image".to_string());
        self
    }
}
