use futures::future::join_all;
use regex::Regex;
use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() {
    // List of addresses to query
    let addresses: Vec<String> = [
        "https://github.com/mrmangohands/lithium-fabric/releases",
        "https://github.com/mrmangohands/sodium-fabric/releases",
        "https://github.com/mrmangohands/phosphor-fabric/releases",
        "https://github.com/mrmangohands/krypton",
    ]
    .iter()
    .map(|&str| str.to_string())
    .collect();

    let bodies = run(addresses).await;
    println!("{:?}", bodies);
}

async fn run(addresses: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    // For each address, send http request to server, do so in the form of await all (in parallel) let client = reqwest::Client::new(); let results = join_all(addresses.iter().map(|&a| client.get(a).send())).await;
    let client = reqwest::Client::new();

    let responses = join_all(
        addresses
            .into_iter()
            .map(|address| client.get(address).send()),
    )
    .await;

    let tentative_bodies = join_all(responses.into_iter().filter_map(|res| match res {
        Ok(res) => Some(res.text()),
        Err(_) => None,
    }))
    .await;

    let bodies: Vec<String> = tentative_bodies
        .into_iter()
        .filter_map(|body| match body {
            Ok(body) => Some(body),
            Err(_) => None,
        })
        .collect();

    let folder = "webpages";
    fs::create_dir_all(folder)?;

    for (i, body) in bodies.iter().enumerate() {
        match fs::write(format!("{}/{}.html", folder, i), body) {
            Ok(_) => {
                println!("Successfully wrote {}.html", i)
            }
            Err(_) => {
                println!("Error writing {}.html to {}", i, folder)
            }
        }
    }

    // The base github url probably won't change
    let github = "https://github.com";
    // The base jar is going to look something like this
    let base = "";

    // Assume that the name may change
    // We use the very first href with a .jar in it to determine the link of which we will get our download

    // In that, we search for a date range
    //

    // temp.into_iter().enumerate().map(|(i,body)| match fs::write(format!("{}/{}.html", folder, i), body) {
    //     Ok(_) => {println!("Successfully wrote {}.html", i)},
    //     Err(_) => {println!("Error writing {}.html to {}", i, folder)}
    // });

    // As for the vector of results, we just want to print the results of the html file for now
    Ok(bodies)
}

// Takes in our html and returns the api link to download the latest file, if needed
fn get_href_from_html(html: String) -> Option<String> {
    // If the regex returned ok
    if let Ok(href_pattern) = Regex::new(r"/(.*.jar)") {
        // Then find the href inside the html!
        if let Some(href) = href_pattern.captures(&html) {
            // Then return the href!
            if let Some(href) = href.get(0) {
                Some(href.as_str().to_string())
            } else {
                println!("There was some error turning the captured href into string");
                None
            }
        } else {
            println!("Couldn't find a link inside the html...is the website broken?");
            None
        }
    } else {
        println!("There was some error initiating regex...");
        None
    }
}

fn get_date_from_match(href: String) -> Vec<String> {
    let date_pattern = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
    let captures = date_pattern.captures(&href).unwrap();
    // [captures.name("year")?, captures.name("month"), captures.name("day")]
    if let (Some(year), Some(month), Some(day)) = (
        captures.name("year"),
        captures.name("month"),
        captures.name("day"),
    ) {
        [year, month, day]
            .iter()
            .map(|matched| matched.as_str().to_string())
            .collect()
    } else {
        println!("Error occured in getting year, month, and date matches from href");
        panic!();
    }
}

struct Date {
    day: u8,
    month: u8,
    year: u32,
}

impl Date {
    fn new(day: u8, month: u8, year: u32) -> Self {
        Date { day, month, year }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // fn new () {
    //     let expected_date = Date {
    //         day: 12, month: 2, year: 2021
    //     };
    //     assert_eq!(get_href_from_html()), Some(expected_date));
    // }

    #[test]
    fn simple_href_from_string() {
        let href = "/mrmangohands/lithium-fabric/releases/download/mc1.16.1-0.6.3-SNAPSHOT%2B2021-02-12/lithium-1.16.1-backport-fabric-0.6.3-SNAPSHOT+2021-02-12.jar";
        let html = r#"
        .StyledCounter_19524_secondary {
          color: var(--color-counter-secondary-text); }
        </style>
              </div>
            </summary>
            <div class="Box Box--condensed mt-3">
              <div>
                  <div class="d-flex flex-justify-between flex-items-center py-1 py-md-2 Box-body px-2">
                    <a href="/mrmangohands/lithium-fabric/releases/download/mc1.16.1-0.6.3-SNAPSHOT%2B2021-02-12/lithium-1.16.1-backport-fabric-0.6.3-SNAPSHOT+2021-02-12.jar" rel="nofollow" class="d-flex flex-items-center min-width-0">
                      <svg class="octicon octicon-package flex-shrink-0 color-text-secondary" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8.878.392a1.75 1.75 0 00-1.756 0l-5.25 3.045A1.75 1.75 0 001 4.951v6.098c0 .624.332 1.2.872 1.514l5.25 3.045a1.75 1.75 0 001.756 0l5.25-3.045c.54-.313.872-.89.872-1.514V4.951c0-.624-.332-1.2-.872-1.514L8.878.392zM7.875 1.69a.25.25 0 01.25 0l4.63 2.685L8 7.133 3.245 4.375l4.63-2.685zM2.5 5.677v5.372c0 .09.047.171.125.216l4.625 2.683V8.432L2.5 5.677zm6.25 8.271l4.625-2.683a.25.25 0 00.125-.216V5.677L8.75 8.432v5.516z"></path></svg>
                      <span class="pl-2 flex-auto min-width-0 text-bold">lithium-1.16.1-backport-fabric-0.6.3-SNAPSHOT+2021-02-12.jar</span>
                    </a>
                    <small class="pl-2 color-text-secondary flex-shrink-0">306 KB</small>
                  </div>


                  <div class="d-block py-1 py-md-2 Box-body px-2">
        "#;
        assert_eq!(get_href_from_html(html.to_string()), Some(href.to_string()))
    }
}

// Highlight one for crit A and one for crit B, otherwise don't do much else
