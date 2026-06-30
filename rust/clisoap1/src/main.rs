// rust/clisoap1/src/main.rs
// Uso: cargo run  -> http://localhost:6001/?n=10
// Consume el servicio SOAP público NumberConversion con reqwest + XML manual

use std::collections::HashMap;
use axum::{extract::Query, response::IntoResponse, routing::get, Router};
use reqwest::Client;
use tokio::net::TcpListener;

const SOAP_URL: &str =
    "https://www.dataaccess.com/webservicesserver/NumberConversion.wso";

fn soap_envelope(n: u64) -> String {
    format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
  <soap:Body>
    <NumberToWords xmlns="http://www.dataaccess.com/webservicesserver/">
      <ubiNum>{}</ubiNum>
    </NumberToWords>
  </soap:Body>
</soap:Envelope>"#,
        n
    )
}

fn extract_result(xml: &str) -> String {
    println!("=== Respuesta SOAP recibida ===");
    println!("{}", xml);
    println!("===============================");
    
    // Buscar de varias formas posibles
    let patterns = vec![
        "<NumberToWordsResult>",
        "<m:NumberToWordsResult>",
        "<ns:NumberToWordsResult>",
    ];
    
    for pattern in patterns {
        if let (Some(start_idx), Some(end_idx)) = (xml.find(pattern), xml.find(&format!("</{}", &pattern[1..]))) {
            let result_start = start_idx + pattern.len();
            let result = xml[result_start..end_idx].trim().to_string();
            if !result.is_empty() {
                println!("Resultado extraído: {}", result);
                return result;
            }
        }
    }
    
    // Si no encuentra nada, devolver el XML completo para debug
    println!("No se pudo extraer el resultado");
    "error".to_string()
}

async fn handler(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let n: u64 = params.get("n").and_then(|v| v.parse().ok()).unwrap_or(10);
    let client = Client::new();
    let body = soap_envelope(n);

    println!("Enviando SOAP para n={}", n);

    match client
        .post(SOAP_URL)
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("SOAPAction", "\"NumberToWords\"")
        .body(body)
        .send()
        .await
    {
        Ok(resp) => {
            match resp.text().await {
                Ok(xml) => extract_result(&xml),
                Err(e) => {
                    eprintln!("Error al leer respuesta: {}", e);
                    "error".to_string()
                }
            }
        }
        Err(e) => {
            eprintln!("Error en request SOAP: {}", e);
            "error".to_string()
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let listener = TcpListener::bind("0.0.0.0:6001").await.unwrap();
    println!("http://localhost:6001/?n=10");
    axum::serve(listener, app.into_make_service()).await.unwrap();
}