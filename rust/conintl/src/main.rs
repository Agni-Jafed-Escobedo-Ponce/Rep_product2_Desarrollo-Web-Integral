// rust/conintl/src/main.rs
// Uso: cargo run  -> http://localhost:6003/?n=10
// Convierte un número a letras en español (implementación propia)

use std::collections::HashMap;
use axum::{extract::Query, response::IntoResponse, routing::get, Router};
use tokio::net::TcpListener;

static UNIDADES: &[&str] = &[
    "", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve",
    "diez", "once", "doce", "trece", "catorce", "quince",
    "dieciséis", "diecisiete", "dieciocho", "diecinueve",
];
static DECENAS: &[&str] = &[
    "", "", "veinte", "treinta", "cuarenta", "cincuenta",
    "sesenta", "setenta", "ochenta", "noventa",
];
static CENTENAS: &[&str] = &[
    "", "ciento", "doscientos", "trescientos", "cuatrocientos", "quinientos",
    "seiscientos", "setecientos", "ochocientos", "novecientos",
];

fn convertir_es(n: i64) -> String {
    if n == 0   { return "cero".to_string(); }
    if n < 0    { return format!("menos {}", convertir_es(-n)); }
    if n == 100 { return "cien".to_string(); }
    if n < 20   { return UNIDADES[n as usize].to_string(); }
    if n < 100  {
        let (d, u) = (n / 10, n % 10);
        if u == 0 { return DECENAS[d as usize].to_string(); }
        if d == 2 { return format!("veinti{}", UNIDADES[u as usize]); }
        return format!("{} y {}", DECENAS[d as usize], UNIDADES[u as usize]);
    }
    if n < 1_000 {
        let (c, r) = (n / 100, n % 100);
        if r == 0 { return CENTENAS[c as usize].to_string(); }
        return format!("{} {}", CENTENAS[c as usize], convertir_es(r));
    }
    if n < 2_000 {
        let r = n % 1_000;
        if r == 0 { return "mil".to_string(); }
        return format!("mil {}", convertir_es(r));
    }
    if n < 1_000_000 {
        let (m, r) = (n / 1_000, n % 1_000);
        let base = format!("{} mil", convertir_es(m));
        if r == 0 { return base; }
        return format!("{} {}", base, convertir_es(r));
    }
    let (m, r) = (n / 1_000_000, n % 1_000_000);
    let base = if m == 1 {
        "un millón".to_string()
    } else {
        format!("{} millones", convertir_es(m))
    };
    if r == 0 { return base; }
    format!("{} {}", base, convertir_es(r))
}

async fn handler(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let n: i64 = params.get("n").and_then(|v| v.parse().ok()).unwrap_or(10);
    convertir_es(n)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let listener = TcpListener::bind("0.0.0.0:6003").await.unwrap();
    println!("http://localhost:6003/?n=10");
    axum::serve(listener, app.into_make_service()).await.unwrap();
}