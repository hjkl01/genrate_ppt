use anyhow::Result;

pub async fn search_image(keyword: &str) -> Result<Vec<String>> {
    // Placeholder for image search provider integration.
    Ok(vec![format!("image://{}", keyword)])
}
