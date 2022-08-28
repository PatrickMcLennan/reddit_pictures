use std::{collections, fs};
use scraper::{Html, Selector};
use dotenv;
use std::io::prelude::*;
use collections::HashMap;

struct UrlAndName {
    url: String,
    name: String
}

#[tokio::main]
async fn main() {
    let url = dotenv::var("URL").expect("No URL was found in the env");
    let path = std::sync::Arc::new(
        dotenv::var("DIR_PATH").expect("No DIR_PATH was found in the env")
    );
    
    let html = reqwest::get(url)
        .await
        .expect("Reqwest can't unwrap response from URL")
        .text()
        .await
        .expect("Can't parse the reqwest response into text");
	let document = Html::parse_document(&html);

    let file_map: HashMap<String, fs::DirEntry> =
        fs::read_dir(&*path.clone())
            .expect("Can't read files from PATH")
            .fold(HashMap::new(), |mut map, res| {
                if let Ok(res) = res {
                    map.insert(
                        res
                            .file_name()
                            .to_str()
                            .expect("Can't get a file name")
                            .to_string(), 
                        res
                    );
                    map
                } else {
                    map
                }
            });

    let posts_selector = &Selector::parse(
        r#"div.thing:not([data-promoted='true']):not([data-nsfw='true'])"#
    ).expect("Can't create parser for posts");

    let posts = document
        .select(posts_selector)
        .filter_map(|post| {
			let url = match post.value().attr("data-url") {
				Some(v) => v.to_string(),
				None => return None
			};
			let name = match url.split("/").collect::<Vec<&str>>().last() {
				Some(v) => v.to_string(),
				None => return None
			};
			if url.contains("https://i.redd.it") && !file_map.contains_key(&name) { 
				println!("No match found for {}, downloading now", &name);
				Some(UrlAndName {
					url,
					name
				}) 
			} else { None }
        })
        .collect::<Vec<UrlAndName>>();

    let downloads = futures::stream::FuturesUnordered::new();
    for post in posts {
        let path_ref = path.clone();
        downloads.push(tokio::spawn(async move {
            let full_path = format!("{}/{}", path_ref, post.name);
            let mut file = std::fs::File::create(full_path)
                .expect(&format!("Can't create a file for {}", post.name));
            let res = reqwest::get(&post.url)
                .await
                .expect(&format!("Got no response trying to download {}", post.url))
                .bytes()
                .await
                .expect(&format!("Can't get bytes for {}", post.url));
            file.write_all(&res)
        }))
    }

    futures::future::join_all(downloads).await;
    ()
}
