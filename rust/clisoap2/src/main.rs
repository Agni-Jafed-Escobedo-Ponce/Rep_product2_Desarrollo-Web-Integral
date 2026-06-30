// rust/clisoap2/src/main.rs
// Uso: cargo run  -> http://localhost:6002/?n=10
// Consume el servicio SOAP y traduce al español con la API de Google Translate

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
                println!("Resultado SOAP extraído: {}", result);
                return result;
            }
        }
    }
    
    println!("No se pudo extraer el resultado del SOAP");
    "error".to_string()
}

async fn traducir(texto: &str) -> String {
    println!("=== Traduciendo texto: {} ===", texto);
    
    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=es&dt=t&q={}",
        urlencoding::encode(texto)
    );
    
    println!("URL de traducción: {}", url);
    
    let client = Client::new();
    
    match client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await
    {
        Ok(resp) => {
            match resp.text().await {
                Ok(response_text) => {
                    println!("=== Respuesta de Google Translate ===");
                    println!("{}", response_text);
                    println!("======================================");
                    
                    // Intentar extraer la traducción
                    if let Some(inicio) = response_text.find("[[\"") {
                        let inicio = inicio + 3;
                        if let Some(fin) = response_text[inicio..].find('"') {
                            let traduccion = response_text[inicio..inicio + fin].to_string();
                            println!("Traducción extraída: {}", traduccion);
                            return traduccion;
                        }
                    }
                    
                    println!("No se pudo parsear la respuesta de traducción");
                    format!("{} (sin traducción)", texto)
                }
                Err(e) => {
                    eprintln!("Error al leer respuesta de traducción: {}", e);
                    format!("{} (error de traducción)", texto)
                }
            }
        }
        Err(e) => {
            eprintln!("Error en request de traducción: {}", e);
            format!("{} (error de traducción)", texto)
        }
    }
}

async fn handler(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let n: u64 = params.get("n").and_then(|v| v.parse().ok()).unwrap_or(10);
    let client = Client::new();
    let body = soap_envelope(n);

    println!("\n=== Iniciando conversión para n={} ===", n);
    println!("Enviando SOAP...");

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
                Ok(xml) => {
                    let en = extract_result(&xml);
                    
                    if en == "error" {
                        return "error".to_string();
                    }
                    
                    println!("Texto en inglés: {}", en);
                    let es = traducir(&en).await;
                    println!("Texto en español: {}", es);
                    println!("===================================\n");
                    
                    es
                }
                Err(e) => {
                    eprintln!("Error al leer respuesta SOAP: {}", e);
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
    let listener = TcpListener::bind("0.0.0.0:6002").await.unwrap();
    println!("http://localhost:6002/?n=10");
    axum::serve(listener, app.into_make_service()).await.unwrap();
}