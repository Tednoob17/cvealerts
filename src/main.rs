use reqwest::blocking::get;
use std::env;
use docx::document::Paragraph;
use docx::Docx;
extern crate serde;
extern crate serde_json;

#[derive(Debug, serde::Deserialize)]
struct {


}

fn ft_search_cves(parameters: &str) -> Result<(), reqwest::Error> {
    let base_url = "https://services.nvd.nist.gov/rest/json/cves/2.0";
    let response = get(&format!("{}?{}", base_url, parameters))?;

    if response.status().is_success() {
        let json_data = response.json::<serde_json::Value>()?;
        println!("{:#}", json_data);
    } else {
        eprintln!("Erreur lors de la récupération des données. Code d'état : {}", response.status());
    }

    Ok(())
}

fn save_to_file() {

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let parameters = args[1..].join("&");

    if let Err(e) = ft_search_cves(&parameters) {
        eprintln!("Erreur : {}", e);
    }





}