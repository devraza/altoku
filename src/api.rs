use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub releaseDate: String,
    pub subOrDub: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Search {
    pub currentPage: String,
    pub hasNextPage: bool,
    pub results: Vec<SearchResult>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Quality {
    pub url: String,
    pub quality: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Watch {
    pub sources: Vec<Quality>,
    pub download: String,
}

pub async fn query_anime(query: String) -> Result<Search, reqwest::Error> {
    reqwest::get(format!(
        "https://altoku-api.vercel.app/anime/gogoanime/{}?page=1",
        query
    ))
    .await
    .expect("Failed to get from API")
    .json::<Search>()
    .await
}

pub async fn get_url(id: String) -> Result<Watch, reqwest::Error> {
    reqwest::get(format!(
        "https://altoku-api.vercel.app/anime/gogoanime/watch/{}",
        id
    ))
    .await
    .expect("Failed to get from API")
    .json::<Watch>()
    .await
}
