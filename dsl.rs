use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresentationSpec {
    pub title: String,
    #[serde(default)]
    pub theme: String,
    pub slides: Vec<SlideSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlideSpec {
    pub id: String,
    pub kind: SlideKind,
    pub title: String,
    #[serde(default)]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub components: Vec<Component>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlideKind {
    Cover,
    Section,
    Content,
    Comparison,
    Architecture,
    Timeline,
    Summary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Component {
    Text { text: String, #[serde(default)] role: TextRole },
    Card { title: String, #[serde(default)] body: Vec<String> },
    Image { prompt: String, #[serde(default)] alt: String },
    Node { id: String, label: String },
    Connector { from: String, to: String, #[serde(default)] arrow: bool },
    TimelineItem { label: String, description: String },
    Metric { label: String, value: String, #[serde(default)] detail: Option<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TextRole { #[default] Body, Heading, Caption, Bullet }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub topic: String,
    #[serde(default)]
    pub audience: Option<String>,
    #[serde(default)]
    pub style: Option<String>,
    #[serde(default = "default_slide_count")]
    pub slide_count: usize,
}

fn default_slide_count() -> usize { 8 }

impl PresentationSpec {
    pub fn validate(&self, expected_slides: usize) -> anyhow::Result<()> {
        if self.title.trim().is_empty() { anyhow::bail!("presentation title is empty"); }
        if self.slides.is_empty() { anyhow::bail!("presentation has no slides"); }
        if self.slides.len() > expected_slides.saturating_add(2) {
            anyhow::bail!("planner returned too many slides: {}", self.slides.len());
        }
        for (index, slide) in self.slides.iter().enumerate() {
            if slide.id.trim().is_empty() { anyhow::bail!("slide {} has empty id", index + 1); }
            if slide.title.trim().is_empty() { anyhow::bail!("slide {} has empty title", index + 1); }
            if slide.components.len() > 12 { anyhow::bail!("slide {} has too many components", index + 1); }
        }
        Ok(())
    }
}
