use crate::{Result, utils::PasswordGenerator};
use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::{cors::CorsLayer, services::ServeDir};

/// Web server for PassMan-CLI
pub struct WebServer {
    port: u16,
}

impl WebServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    /// Start the web server
    pub async fn serve(self) -> Result<()> {
        let app = self.create_app();

        let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .await
            .map_err(|e| crate::Error::Io(e))?;

        println!("🚀 PassMan-CLI Web UI running at http://127.0.0.1:{}", self.port);
        println!("📝 Open your browser and go to the URL above to use the web interface");

        axum::serve(listener, app)
            .await
            .map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        Ok(())
    }

    fn create_app(&self) -> Router {
        Router::new()
            .route("/", get(home_handler))
            .route("/api/generate", post(generate_password_handler))
            .route("/api/passwords", get(list_passwords_handler))
            .route("/api/passwords", post(add_password_handler))
            .layer(CorsLayer::permissive())
    }
}

#[derive(Deserialize)]
struct GeneratePasswordRequest {
    length: Option<u32>,
    include_symbols: Option<bool>,
    include_numbers: Option<bool>,
    include_uppercase: Option<bool>,
    include_lowercase: Option<bool>,
}

#[derive(Serialize)]
struct GeneratePasswordResponse {
    password: String,
    length: usize,
}

#[derive(Serialize)]
struct PasswordEntry {
    id: String,
    title: String,
    username: String,
    url: Option<String>,
    notes: Option<String>,
    created_at: String,
}

#[derive(Deserialize)]
struct AddPasswordRequest {
    title: String,
    username: String,
    password: String,
    url: Option<String>,
    notes: Option<String>,
}

/// Home page handler
async fn home_handler() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}

/// Generate password API endpoint
async fn generate_password_handler(
    Json(req): Json<GeneratePasswordRequest>,
) -> std::result::Result<Json<GeneratePasswordResponse>, StatusCode> {
    let mut config = crate::utils::GeneratorConfig::default();
    
    if let Some(length) = req.length {
        config.length = length;
    }
    if let Some(symbols) = req.include_symbols {
        config.include_symbols = symbols;
    }
    if let Some(numbers) = req.include_numbers {
        config.include_numbers = numbers;
    }
    if let Some(uppercase) = req.include_uppercase {
        config.include_uppercase = uppercase;
    }
    if let Some(lowercase) = req.include_lowercase {
        config.include_lowercase = lowercase;
    }

    let generator = PasswordGenerator::with_config(config);
    match generator.generate() {
        Ok(password) => {
            let length = password.len();
            Ok(Json(GeneratePasswordResponse { password, length }))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// List passwords API endpoint
async fn list_passwords_handler() -> Json<Vec<PasswordEntry>> {
    // Mock data for now
    let passwords = vec![
        PasswordEntry {
            id: "1".to_string(),
            title: "GitHub".to_string(),
            username: "user@example.com".to_string(),
            url: Some("https://github.com".to_string()),
            notes: Some("Work account".to_string()),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        },
        PasswordEntry {
            id: "2".to_string(),
            title: "Gmail".to_string(),
            username: "personal@gmail.com".to_string(),
            url: Some("https://gmail.com".to_string()),
            notes: None,
            created_at: "2024-01-02T00:00:00Z".to_string(),
        },
    ];
    
    Json(passwords)
}

/// Add password API endpoint
async fn add_password_handler(
    Json(req): Json<AddPasswordRequest>,
) -> std::result::Result<Json<PasswordEntry>, StatusCode> {
    // Mock implementation - in real app, this would save to database
    let entry = PasswordEntry {
        id: "new".to_string(),
        title: req.title,
        username: req.username,
        url: req.url,
        notes: req.notes,
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    Ok(Json(entry))
}
