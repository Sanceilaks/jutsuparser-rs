use std::collections::HashMap;

use kuchiki::{traits::TendrilSink, NodeRef};

pub(crate) fn create_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X x.y; rv:42.0) Gecko/20100101 Firefox/42.0")
        .build()
        .expect("Cannot create web client due to")
}

pub(crate) async fn get_parsed(url: &str) -> NodeRef {
    let client = create_http_client();
    let resp = client.get(url).send().await.unwrap();

    let response = resp.error_for_status().expect("Cannot get episode page");
    let page_content = response.text().await.unwrap();
    kuchiki::parse_html().one(page_content)
}

pub struct Anime {
    pub name: String,
    pub url: String,
    pub image_url: String
}

pub struct Episode {
    pub url: String
}

impl Episode {
    pub fn new(url: &str) -> Episode {
        Episode { url: (url.to_string()) }
    }

    pub async fn get_name(&self) -> String {
        get_parsed(&self.url).await.select_first(".video_plate_title")
            .unwrap().as_node().select_first("h2").unwrap()
                .text_contents()
    }

    pub fn get_episode_index(&self) -> usize {
        let regex = regex::Regex::new(r"/episode-(\d+)\.html").unwrap();
        let out: usize = regex.captures_iter(&self.url).next().and_then(|n|{
            Some(n.get(1).unwrap().as_str().parse::<usize>().unwrap())
        }).unwrap();
        out
    }
}

impl Anime {
    ///
    /// Search for anime by his name
    /// 
    pub async fn search(anime_name: &str) -> Result<Vec<Anime>, String> {
        let client = create_http_client();
        let mut params = HashMap::new();
        params.insert("ajax_load", "yes");
        params.insert("start_from_page", "1");
        params.insert("show_search", anime_name);
        params.insert("anime_of_user", "");
    
        let request = match client.post("https://jut.su/anime")
            .form(&params)
            .header("Referer", "http://jut.su/anime/")
            .send().await {
                Ok(o) => o,
                Err(why) => {
                    return Err(std::format!("Cannot get response with error {:?}", why))
                }
        };
        
        let response = match request.error_for_status() {
            Ok(res) => res,
            Err(why) => {
                return Err(std::format!("Cannot get response with error  {:?}", why));
            }
        };
    
        let page_content = response.text().await.expect("Cannot get page content");
        let document = kuchiki::parse_html().one(page_content);
    
        let mut anime : Vec<Anime> = Vec::new();
    
        document.select("div.all_anime_global").unwrap().for_each(|node| {
            let as_node = node.as_node();
            
            let body = as_node.select_first("div.all_anime").unwrap();
    
            let url: String = as_node.select_first("a").and_then(|n| {
                let node_n = n.attributes.borrow();
                Ok(node_n.get("href").unwrap().to_string())
            }).expect("Cannot get URL");
    
            let image_style = body.as_node().select_first(".all_anime_image").and_then(|n| {
                let attrib = n.attributes.borrow();
                Ok(attrib.get("style").unwrap().to_string())
            }).expect("Cannot get image style");
    
            let regex = regex::Regex::new(r"'(https://.*?)'").unwrap();
            let image = regex.captures_iter(image_style.as_str()).next().and_then(|m| {
                Some(m.get(1).unwrap().as_str().to_string())
            }).expect("Cannot get image");
    
            let name = body.as_node().select_first(".aaname").and_then(|aaname|{
                Ok(aaname.as_node().text_contents())
            }).expect("Cannot get anime name");
    
            anime.push(Anime {
                 name: (name), url: (format!("https://jut.su{}", url)), image_url: (image) 
            });
        });
    
        Ok(anime)
    }
    
    pub async fn get_description(&self) -> String {
        get_parsed(&self.url).await.select_first("#dle-content > div > div:nth-child(2) > div > p > span")
            .unwrap().text_contents()
    }

    pub async fn get_genres(&self) -> Vec<String> {
        get_parsed(&self.url).await.select("#dle-content > div > div:nth-child(2) > div > div > a")
            .unwrap().map(|el| el.text_contents()).collect()
    }
    
    pub async fn get_name(&self) -> String {
        let text = get_parsed(&self.url).await
            .select_first("h1.header_video").unwrap().text_contents();
        regex::Regex::new(r"Смотреть (.+) все серии").unwrap().
            captures_iter(text.as_str()).
                next().unwrap().get(1).unwrap().as_str().to_owned()
    }

    pub async fn get_original_name(&self) -> String {
        get_parsed(&self.url).await.select_first("#dle-content > div > div:nth-child(2) > div > div > b")
            .unwrap().text_contents()
    }

    pub async fn get_episodes(&self) -> Vec<Episode> {
        let client = create_http_client();

        let resp = client.get(&self.url).send().await
            .expect("Cannot get anime page content");
        
        let response = resp.error_for_status()
            .expect("Cannot get anime page content");

        let page_content = response.text().await.expect("Cannot get page content");
        let document = kuchiki::parse_html().one(page_content);

        let mut episodes: Vec<Episode> = Vec::new();

        document.select(".video")
            .expect("Cannot find .video elements").for_each(|node| {
                let url = node.attributes.try_borrow().and_then(|attrs|{
                    Ok(attrs.get("href").expect("Cannot find href attribute").to_string())
                }).unwrap();
                
                episodes.push(Episode {
                    url: (format!("http://jut.su{}", url)) 
                });
            });

        episodes
    }
}