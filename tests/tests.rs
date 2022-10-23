

#[cfg(test)]
mod anime_list_tests {
    #[tokio::test]
    async fn get_anime_test() {
        let animes = jutsuparser_rs::search("Две звезды")
            .await
            .expect("Cannot get anime");
        let first_anime = animes.iter().next().expect("Cannot get anime");
        assert_eq!(first_anime.url, "https://jut.su/onmyouji/")
    }
}