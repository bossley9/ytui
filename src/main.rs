extern crate reqwest;

use json;
use regex::Regex;
use std::env;

const RED: &str = "\x1b[31m";
const NC: &str = "\x1b[0m";

const FQDN: &str = "https://youtube.com";
const PATH_SEARCH_RESULTS: &str = "/results?search_query=";

struct Video {
    id: String,
    name: String,
    thumbnail: String,
    desc: String,
    channel_name: String,
    channel_url: String,
    published: String,
    length: String,
    is_live: bool,
    views: u64,
    url: String,
}

// struct Playlist {
//     channel: Channel,
//     name: String,
//     desc: String,
//     num_videos: u16,
//     videos: Vec<Video>,
// }

// struct Channel {
//     id: String,
//     subscribers: u32,
//     name: String,
//     videos: Vec<Video>,
//     playlists: Vec<Playlist>,
// }

fn encode(unencoded_url: &mut String) {
    *unencoded_url = str::replace(unencoded_url, "%", "%25");
    *unencoded_url = str::replace(unencoded_url, " ", "%20");
    *unencoded_url = str::replace(unencoded_url, "'", "%91");
}

#[tokio::main]
async fn main() {
    // search command line args
    let args = env::args().skip(1);

    let mut search_str: String = args
        .fold(String::new(), |acc, el| acc + &el + " ")
        .trim()
        .to_string();

    if search_str.len() == 0 {
        println!("{}USAGE: ytui [search query]{}", RED, NC);
        return;
    }

    // encode search query
    encode(&mut search_str);

    // construct url
    let mut url = String::from(FQDN);
    url.push_str(PATH_SEARCH_RESULTS);
    url.push_str(&search_str);

    // fetch data (with error handling)
    let res: reqwest::Response = match reqwest::get(&url).await {
        Ok(res) => res,
        Err(_) => return,
    };

    let mut data = match res.text().await {
        Ok(res) => res,
        Err(_) => "".to_string(),
    };

    // 1. condense to a single line (remove new line chars)
    data = str::replace(&data, "\n", "");
    // 2. trim start up to "var ytInitialData = "
    let re = Regex::new(r"^.*var\s+ytInitialData\s+=\s+").unwrap();
    data = re.replace(&data, "").to_string();
    // 3. trim end starting from "</script>"
    let re = Regex::new(r";?</script>.*$").unwrap();
    data = re.replace(&data, "").to_string();
    // 4. parse json
    let parsed = match json::parse(&data) {
        Ok(res) => res,
        Err(_) => json::JsonValue::new_object(),
    };

    let content = &parsed["contents"]["twoColumnSearchResultsRenderer"]["primaryContents"]
        ["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"];

    // println!("{}", content);

    let mut videos: Vec<Video> = Vec::new();

    for i in content.members() {
        let video_raw = &i["videoRenderer"];

        if !video_raw.is_empty() {
            let id: String = video_raw["videoId"].to_string();

            let thumbnail: String = video_raw["thumbnail"]["url"].to_string();

            // TODO join
            let name = video_raw["title"]["runs"][0]["text"].to_string();

            // TODO join
            let desc = video_raw["descriptionSnippet"]["runs"][0]["text"].to_string();

            // join?
            let channel_name = video_raw["ownerText"]["runs"][0]["text"].to_string();

            let mut channel_url = String::from(FQDN);
            let path_channel_url = video_raw["ownerText"]["runs"][0]["navigationEndpoint"]
                ["commandMetadata"]["webCommandMetadata"]["url"]
                .to_string();
            channel_url.push_str(&path_channel_url);
            let mut published = "".to_string();
            if !video_raw["publishedTimeText"].is_empty() {
                published = video_raw["publishedTimeText"]["simpleText"].to_string();
            }

            let mut is_live = false;
            let live = &video_raw["viewCountText"]["runs"];
            if !live.is_empty() {
                is_live = true;
            }

            let length = if is_live {
                "live".to_string()
            } else {
                video_raw["lengthText"]["simpleText"].to_string()
            };

            let mut views_raw = String::new();
            if !is_live {
                views_raw = video_raw["viewCountText"]["simpleText"].to_string();
                let mut re = Regex::new(r"\sviews").unwrap();
                views_raw = re.replace(&views_raw, "").to_string();
                re = Regex::new(r",").unwrap();
                views_raw = re.replace_all(&views_raw, "").to_string();
            } else {
                views_raw = video_raw["viewCountText"]["runs"][0]["text"].to_string();
            }

            let views = match views_raw.parse::<u64>() {
                Ok(num) => num,
                Err(_) => 0,
            };

            let mut url = String::from(FQDN);
            url.push_str("/watch?v=");
            url.push_str(&id);

            // println!("videoRenderer is {}", video_raw);

            videos.push(Video {
                id,
                name,
                thumbnail,
                desc,
                channel_name,
                channel_url,
                published,
                length,
                views,
                is_live,
                url,
            })
        }
    }

    // return results
    for v in videos.iter() {
        match v {
            Video {
                length,
                channel_name,
                name,
                views,
                url,
                ..
            } => println!(
                "[{}]\t{} - {} ({} views) - {}",
                length, channel_name, name, views, url
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // routine dummy test
    #[test]
    fn test_test() {
        assert_eq!(1 + 2, 3);
    }
}
