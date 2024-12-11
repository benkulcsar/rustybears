use reqwest::Error;
use serde::Deserialize;
use serde_json::json;
use std::env;
use thousands::Separable;

#[derive(Deserialize, Debug)]
struct PackageData {
    total_downloads: u64,
}

async fn fetch_downloads(package_name: &str) -> Result<u64, Error> {
    let url = format!("https://pepy.tech/api/v2/projects/{}", package_name);
    let response: PackageData = reqwest::get(&url).await?.json().await?;
    Ok(response.total_downloads)
}

async fn update_gist(content: &serde_json::Value) -> Result<(), Error> {
    let gist_pat = env::var("GH_GIST_PAT").expect("GH_GIST_PAT is not set");
    let gist_id = env::var("RUSTYBEARS_GIST_ID").expect("RUSTYBEARS_GIST_ID is not set");

    let url = format!("https://api.github.com/gists/{}", gist_id);
    let client = reqwest::Client::new();

    let body = json!({
        "files": {
            "rustybears.json": {
                "content": content.to_string()
            }
        }
    });

    let response = client
        .patch(&url)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", gist_pat))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "rustybears")
        .json(&body)
        .send()
        .await?;

    let status = response.status();
    println!("Response Status: {}", status);

    if !status.is_success() {
        eprintln!("Gist update failed with status: {}", status);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let packages = ["pandas", "polars"];
    let mut total_downloads = 0;
    let mut polars_downloads = 0;

    let mut package_data = serde_json::Map::new();

    for package in &packages {
        match fetch_downloads(package).await {
            Ok(downloads) => {
                package_data.insert(
                    package.to_string(),
                    json!(format!(
                        "{} million total downloads",
                        (downloads / 1_000_000).separate_with_commas()
                    )),
                );
                total_downloads += downloads;
                if *package == "polars" {
                    polars_downloads = downloads;
                }
            }
            Err(e) => println!("Failed to fetch data for '{}': {}", package, e),
        }
    }

    if total_downloads > 0 {
        let polars_ratio = (polars_downloads as f64 / total_downloads as f64) * 100.0;
        let pandas_ratio = 100.0 - polars_ratio;

        package_data.insert(
            "polars_ratio".to_string(),
            json!(format!("{:.2}%", polars_ratio)),
        );
        package_data.insert(
            "pandas_ratio".to_string(),
            json!(format!("{:.2}%", pandas_ratio)),
        );
    }

    let json_output = json!(package_data);

    println!("{}", serde_json::to_string_pretty(&json_output).unwrap());

    if let Err(e) = update_gist(&json_output).await {
        eprintln!("Failed to update gist: {}", e);
    }
}
