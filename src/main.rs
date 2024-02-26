use reqwest::blocking::get;
use std::env;
use serde_json::Value; // Import the Value type from serde_json
use std::fs::File;
use std::io::{Write, Result};

fn ft_search_cves(parameters: &str) -> Result<()> {
    let base_url = "https://services.nvd.nist.gov/rest/json/cves/2.0";
    let response = get(&format!("{}?{}", base_url, parameters))?;

    if response.status().is_success() {
        let json_data: Value = response.json()?; // Deserialize into serde_json::Value
        let stdout_content = json_data.to_string(); // Convert to a string

        // Save stdout content to a file
        save_stdout_to_file(&stdout_content)?;
    } else {
        eprintln!("Erreur lors de la récupération des données. Code d'état : {}", response.status());
    }

    Ok(())
}

fn save_stdout_to_file(stdout_content: &str) -> Result<()> {
    let mut file = File::create("output.txt")?;
    file.write_all(stdout_content.as_bytes())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let parameters = args[1..].join("&");

    if let Err(e) = ft_search_cves(&parameters) {
        eprintln!("Erreur : {}", e);
    }
}
