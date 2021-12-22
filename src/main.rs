use std::env;
use clap::{Arg, App};
use dotenv::dotenv;
use reqwest::Url;
use reqwest::header::HeaderMap;
use serde_derive::Deserialize;
use exitfailure::ExitFailure;

// https://www.reddit.com/r/rust/comments/7hasv6/comment/dqpht6v/?utm_source=share&utm_medium=web2x&context=3
#[derive(Deserialize, Debug)] struct Items { html_url: String }
#[derive(Deserialize, Debug)] struct Repo { items: Vec<Items> }

impl Repo {
    // https://dev.to/hb/how-to-fetch-a-web-api-with-rust-1390
    async fn get(package: &String, api_key: &String, github_name: &String) -> Result<Self, ExitFailure> {
        // https://stackoverflow.com/a/63629726/10503012
        // https://docs.github.com/en/search-github/searching-on-github/searching-code
        let url = format!(
            "https://api.github.com/search/code?q={package} +in:file +extension:toml",
            package = package
        );

        let url = Url::parse(&*url)?;
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", github_name.parse().unwrap());
        headers.insert("Authorization", api_key.parse().unwrap());

        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .json::<Repo>()
            .await?;

        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    dotenv().ok();

    let matches = App::new("Cargo Package Code")
                                    .version("0.1")
                                    .author("Stef16Robbe <stef.robbe@gmail.com>")
                                    .about("Let's you search Github for repositories containing Cargo packages")
                                    .arg(Arg::with_name("name")
                                        .short("n")
                                        .long("name")
                                        .value_name("NAME")
                                        .help("Set the Cargo package name to search for")
                                        .required(true)
                                        .takes_value(true))
                                    .get_matches();

    // fltk-table
    let cargo_package = matches.value_of("name").unwrap().to_string();
    let api_key = env::var("GITHUB_API_KEY").unwrap();
    let username = env::var("GITHUB_USERNAME").unwrap();

    println!("Searching for Repo's that use the {} in their toml file...", cargo_package);
    let res = Repo::get(&cargo_package, &api_key, &username).await?;
    println!("{:?}", res);

    Ok(())
}
