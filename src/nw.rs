use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://download.freebsd.org/releases/amd64/15.0-RELEASE/kernel.txz")
        .await?;
    println!("{resp:#?}");
    Ok(())
}