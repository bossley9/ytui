use std::env;

const RED: &str = "\x1b[31m";
const NC: &str = "\x1b[0m";

// const FQDN: &str = "https://youtube.com";
// const PATH_SEARCH_RESULTS: &str = "results?search_query=";

fn main() {
    // search command line args
    let args = env::args().skip(1);

    let search_str: String = args
        .fold(String::new(), |acc, el| acc + &el + " ")
        .trim()
        .to_string();

    if search_str.len() == 0 {
        println!("{}USAGE: ytui [search query]{}", RED, NC);
        return;
    }

    println!("search_str is \"{}\"", search_str);

    // encode search query

    // construct url

    // fetch data (with error handling)

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
