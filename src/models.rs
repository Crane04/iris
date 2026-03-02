use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CompareRequest {
    pub target_url: String,
    pub people: Vec<Person>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub name: String,
    pub image_url: String,
}

#[derive(Serialize)]
pub struct MatchResult {
    pub name: String,
    pub probability: f64,
}

#[derive(Serialize)]
pub struct CompareResponse {
    pub matches: Vec<MatchResult>,
}