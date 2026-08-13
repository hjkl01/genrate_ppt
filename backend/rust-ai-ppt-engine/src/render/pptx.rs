use crate::schema::slide::SlideDeck;

pub struct PptxRenderer;

impl PptxRenderer {
    pub fn render(deck: &SlideDeck) -> Vec<u8> {
        // Placeholder renderer. The next step will integrate pptx generation.
        let _ = deck;
        Vec::new()
    }
}
