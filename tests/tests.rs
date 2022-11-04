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
    async fn get_episode_test() {
        let anime = jutsuparser_rs::Anime::search("Две звезды")
            .await
            .expect("Cannot get anime");
        let first_anime = anime.iter().next().expect("Cannot get anime");
        let episodes = first_anime.get_episodes().await;

        match episodes.iter().next() {
            Some(episode) => {
                assert_eq!(episode.get_name().await, "Две судьбы: парень встречает девушку");
                assert_eq!(episode.get_episode_index(), 1);
            },
            _ => {}
        }
    }

    #[tokio::test]
    async fn get_description_test() {
        let anime = jutsuparser_rs::Anime::search("Две звезды")
            .await
            .expect("Cannot get anime");
        let first_anime = anime.iter().next().expect("Cannot get anime");
        assert!(first_anime.get_description().await.starts_with("В могущественной семье экзорцистов"))
    }

    #[tokio::test]
    async fn get_genres_test() {
        let anime = jutsuparser_rs::Anime::search("Две звезды")
            .await
            .expect("Cannot get anime");
        let first_anime = anime.iter().next().expect("Cannot get anime");
        assert!(first_anime.get_genres().await.contains(&"Аниме боевик".into()))
    }

    #[tokio::test]
    async fn get_names_test() {
        let anime = jutsuparser_rs::Anime::search("Две звезды")
            .await
            .expect("Cannot get anime");
        let first_anime = anime.iter().next().expect("Cannot get anime");
        assert_eq!(first_anime.get_name().await, "Две звезды онмёджи");
        assert_eq!(first_anime.get_original_name().await, "Sousei no Onmyouji");
    }
}