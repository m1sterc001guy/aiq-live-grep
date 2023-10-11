#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let server_uri = "http://127.0.0.1:3000/grep";
    let response = reqwest::get(server_uri).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("{body}");
    }

    Ok(())
}
