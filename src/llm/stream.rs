use anyhow::Result;

/// Streaming helper for AI responses.
///
/// The current LLM endpoint returns complete responses. This module defines
/// the internal chunk format so the HTTP SSE layer can be added without
/// changing the React client contract later.
pub struct StreamChunk {
    pub content: String,
    pub done: bool,
}

pub fn split_response(content: &str) -> Result<Vec<StreamChunk>> {
    Ok(content
        .chars()
        .collect::<Vec<_>>()
        .chunks(20)
        .map(|chunk| StreamChunk {
            content: chunk.iter().collect(),
            done: false,
        })
        .collect())
}
