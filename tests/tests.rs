#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn get_anime_test() {
        let anime = jutsuparser_rs::Anime::search("Две звезды")
            .await
            .expect("Cannot get anime");
        let first_anime = anime.iter().next().expect("Cannot get anime");
        assert_eq!(first_anime.url, "https://jut.su/onmyouji/")
    }

    #[tokio::test]
    async fn get_epidoe_test() {
        let anime = jutsuparser_rs::Anime::search("Две звезды")
            .await
            .expect("Cannot get anime");
        let first_anime = anime.iter().next().expect("Cannot get anime");
        let episodes = first_anime.get_epidoes().await;

        assert_eq!(episodes.iter().next().unwrap().get_name().await, "Две судьбы: парень встречает девушку")
    }
}