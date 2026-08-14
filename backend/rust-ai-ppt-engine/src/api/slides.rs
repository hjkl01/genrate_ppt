use axum::Json;
use crate::schema::slide::SlideDeck;

pub async fn get_slides() -> Json<SlideDeck> {
    Json(SlideDeck::default())
}

pub async fn save_slides(Json(deck): Json<SlideDeck>) -> Json<SlideDeck> {
    Json(deck)
}
