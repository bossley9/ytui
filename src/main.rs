use std::env;

const RED: &str = "\x1b[31m";
const NC: &str = "\x1b[0m";

const FQDN: &str = "https://youtube.com";
const PATH_SEARCH_RESULTS: &str = "results?search_query=";

fn encode(unencoded_url: &mut String) {
    *unencoded_url = str::replace(unencoded_url, "%", "%25");
    *unencoded_url = str::replace(unencoded_url, " ", "%20");
    *unencoded_url = str::replace(unencoded_url, "'", "%91");
}

fn main() {
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
    println!("url is \"{}\"", url);

    // 1. condense to a single line (remove new line chars)
    // 2. trim start up to "var ytInitialData = "
    // 3. trim end starting from "</script>"
    // 4. parse json

    // return results
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
