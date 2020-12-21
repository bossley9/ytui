extern crate reqwest;

use json;
use regex::Regex;
use std::env;

const RED: &str = "\x1b[31m";
const NC: &str = "\x1b[0m";

const FQDN: &str = "https://youtube.com";
const PATH_SEARCH_RESULTS: &str = "/results?search_query=";

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

    struct Video {
        id: String,
        title: String,
        thumbnail: String,
        desc: String,
        author: String,
        author_channel: String,
        published: String,
        length: String,
        views: String,
        url: String,
    }

    let mut videos: Vec<Video> = Vec::new();

    for i in content.members() {
        let video_raw = &i["videoRenderer"];

        if !video_raw.is_empty() {
            let id: String = video_raw["videoId"].to_string();
            let thumbnail: String = video_raw["thumbnail"]["url"].to_string();
            // TODO join
            let title = video_raw["title"]["runs"][0]["text"].to_string();
            // TODO join
            let desc = video_raw["descriptionSnippet"]["runs"][0]["text"].to_string();
            // TODO join?
            let author = video_raw["ownerText"]["runs"][0]["text"].to_string();
            let mut author_channel = String::from(FQDN);
            let author_channel_path = video_raw["ownerText"]["runs"][0]["navigationEndpoint"]
                ["commandMetadata"]["webCommandMetadata"]["url"]
                .to_string();
            author_channel.push_str(&author_channel_path);
            let mut published = "".to_string();
            if !video_raw["publishedTimeText"].is_empty() {
                published = video_raw["publishedTimeText"]["simpleText"].to_string();
            }
            let length = video_raw["lengthText"]["simpleText"].to_string();
            let views = video_raw["viewCountText"]["simpleText"].to_string();
            let mut url = String::from(FQDN);
            url.push_str("/watch?v=");
            url.push_str(&id);

            // println!("videoRenderer is {}", video_raw);

            videos.push(Video {
                id,
                title,
                thumbnail,
                desc,
                author,
                author_channel,
                published,
                length,
                views,
                url,
            })
        }
    }

    // return results
    for v in videos.iter() {
        match v {
            Video {
                length,
                author,
                title,
                url,
                ..
            } => println!("({}) {} - {} - {}", length, author, title, url),
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
