use std::env;
use dotenv::dotenv;
use clap::{Arg, App};
use exitfailure::ExitFailure;
use serde_derive::Deserialize;
use reqwest::{Url, StatusCode};
use reqwest::header::HeaderMap;

// https://www.reddit.com/r/rust/comments/7hasv6/comment/dqpht6v/?utm_source=share&utm_medium=web2x&context=3
#[derive(Deserialize, Debug)] struct Repository { html_url: String }
#[derive(Deserialize, Debug)] struct Items { repository: Repository }
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
            .await?;
        
        let res = match res.status() {
            StatusCode::OK => res.json().await?,
            _ => panic!("Failed to request the Github API, looks like you've been rate limited. Status code: {:?}", res.status())
        };

        Ok(res)
    }

    fn print_table(&self) -> Result<String, ExitFailure> {
        let header = "─".repeat(24);
        let mut table: String = format!(
            "{0: <1} │ {1: <20} \n{2}\n",
            " ",
            "Name",
            header
        );

        let mut i = 0;
        for repo in &self.items {
            table += &format!(
                "{0: <1} │ {1: <20}\n",
                i,
                repo.repository.html_url
            );
            i += 1;
        }

        Ok(table)
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

    println!("\nSearching for Repo's that use the {} in their toml file...", cargo_package);
    let res = Repo::get(&cargo_package, &api_key, &username).await?;
    println!("\n{}", Repo::print_table(&res)?);

    Ok(())
}
