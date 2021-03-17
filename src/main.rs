use futures::future::join_all;
use std::error::Error;

#[tokio::main]
async fn main() {
    // List of addresses to query
    let addresses = [
        "https://github.com/mrmangohands/lithium-fabric/releases",
        "https://github.com/mrmangohands/sodium-fabric/releases",
        "https://github.com/mrmangohands/phosphor-fabric/releases",
        "https://github.com/mrmangohands/krypton",
    ];

    run(&addresses).await;
}

async fn run(addresses: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    println!("{:?}", addresses);
    // For each address, send http request to server, do so in the form of await all (in parallel)
    let client = reqwest::Client::new();
    let results = join_all(addresses.iter().map(|&a| client.get(a).send())).await;
    let mut responses_string = Vec::new();
    let responses = join_all(results.iter().filter_map(|result| match result {
        Ok(response) => Some(response.text()),
        Err(_) => None,
    }))
    .await;
    responses.iter().map(|result| match result {
        Ok(res) => responses_string.push(res),
        Err(_) => {}
    });

    // results.iter().filter(|result| match result {
    //     Ok(_) => true,
    //     Err(_) => false
    // }).map(|response|
    //     response.text()
    // );
    // let mut out: Vec<String> = Vec::new();
    // for result in b {
    //     match result {
    //         Ok(res) => out.push(res.text().await?),
    //         Err(_) => (),
    //     }
    // }

    // .iter()
    // .map(|result| match result {
    //     Ok(res) => (*res).text(),
    //     Err(_) =>
    // })
    // ;
    // As for the vector of results, we just want to print the results of the html file for now
    Ok(responses_string)
}

// Highlight one for crit A and one for crit B, otherwise don't do much else
