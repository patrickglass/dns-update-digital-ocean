use digitalocean::prelude::*;
use log::{debug, info};
use reqwest;
use std::env;

#[tokio::main]
async fn main() {
    let domain_name = env::var("DOMAIN_NAME").unwrap_or("swisstech.ca".to_string());
    let api_key = env::var("DO_API_KEY").expect("DO_API_KEY not set.");
    let client = DigitalOcean::new(api_key).unwrap();

    // Fetch the IP address
    let ip = get_ip().await.expect("Could not discover IP Address");

    // print out the ip
    println!("Discovered IP: {}", ip);

    // Fetch the domain records so we can see if the ips match
    let domain = Domain::get(&domain_name.clone())
        .records()
        .execute(&client)
        .expect("Could not fetch domain records");
    debug!("Records: {:#?}", domain);

    // update the domain record if the entry for name "internal" does not match the ip
    for record in domain {
        if record.name().clone() == String::from("van1") {
            if record.data().clone() != ip {
                println!(
                    "Updating domain record for {} from {} to {}",
                    record.name(),
                    record.data(),
                    ip
                );

                let req = Domain::get(&domain_name.clone())
                    .records()
                    .update(record.id().clone())
                    .data(ip.clone());
                info!("Sending request: {:#?}", req);
                let updated_domain = req
                    .execute(&client)
                    .expect("Could not update domain record");

                println!("Updated domain record: {:#?}", updated_domain.name());
                break;
            } else {
                println!(
                    "Domain record for {} is already has the correct value {}",
                    record.name(),
                    ip
                );
            }
        }
    }

    println!("Done!");
}

async fn get_ip() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client.get("https://ipinfo.io/ip")
        .send()
        .await?;
    let resp_text = resp.text().await?;
    info!("IP ADDRESS: {:?}", resp_text);
    Ok(resp_text)
}
