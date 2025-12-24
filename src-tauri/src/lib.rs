#![allow(non_snake_case)]
/**
 * Crunchypi Backend Library.
 * 
 * Handles the main application entry point and Ollama API integration.
 * Exposes commands for the frontend to query the local LLM.
 * 
 * Author: KleaSCM
 * Email: KleaSCM@gmail.com
 */

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct OllamaRequest {
	model: String,
	prompt: String,
	stream: bool,
}

#[derive(Serialize, Deserialize)]
struct OllamaResponse {
	response: String,
}


#[tauri::command]
async fn QueryOllama(prompt: String) -> Result<String, String> {
	// Using reqwest client to make HTTP calls to local Ollama instance.
	// We create a new client for each request to avoid global state complexity, 
	// though connection pooling might be a future optimization.
	let Client = reqwest::Client::new();
	
	let Request = OllamaRequest {
		model: "qwen2-math:1.5b".to_string(),
		prompt,
		stream: false,
	};

	// Return error as string strictly.
	let Response = Client.post("http://localhost:11434/api/generate")
		.json(&Request)
		.send()
		.await
		.map_err(|e| e.to_string())?;

	if !Response.status().is_success() {
		return Err(format!("Request failed with status: {}", Response.status()));
	}

	let Data: OllamaResponse = Response.json()
		.await
		.map_err(|e| e.to_string())?;

	Ok(Data.response)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	// Explicit error handling instead of .expect().
	// If the app fails to run, we log to stderr.
	let Result = tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![QueryOllama])
		.run(tauri::generate_context!());

	if let Err(e) = Result {
		eprintln!("Error while running tauri application: {}", e);
	}
}
