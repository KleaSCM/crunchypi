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
use tauri::{Emitter, Window};
use futures::StreamExt;

#[derive(Serialize, Deserialize)]
struct OllamaRequest {
	model: String,
	prompt: String,
	stream: bool,
}

#[derive(Serialize, Deserialize)]
struct OllamaResponse {
	response: String,
	done: bool,
}

#[tauri::command]
async fn QueryOllama(window: Window, prompt: String) -> Result<String, String> {
	// Using reqwest client to make HTTP calls to local Ollama instance.
	let Client = reqwest::Client::new();
	
	let Request = OllamaRequest {
		model: "qwen2-math:1.5b".to_string(),
		prompt,
		stream: true, // Enable streaming
	};

	// Initiate the request
	let Response = Client.post("http://localhost:11434/api/generate")
		.json(&Request)
		.send()
		.await
		.map_err(|e| e.to_string())?;

	if !Response.status().is_success() {
		return Err(format!("Request failed with status: {}", Response.status()));
	}

	// Process the stream
	let mut Stream = Response.bytes_stream();
	let mut FullResponse = String::new();

	while let Some(ChunkResult) = Stream.next().await {
		let Chunk = ChunkResult.map_err(|e| e.to_string())?;
		let ChunkStr = String::from_utf8_lossy(&Chunk);

		// Ollama sends JSON objects in the stream, sometimes multiple per chunk.
		// We need to parse them.
		for Line in ChunkStr.split('\n') {
			if Line.trim().is_empty() {
				continue;
			}
			
			if let Ok(Data) = serde_json::from_str::<OllamaResponse>(Line) {
				// Emit the token to the frontend
				if let Err(e) = window.emit("ollama-event", Data.response.clone()) {
					eprintln!("Failed to emit event: {}", e);
				}
				
				FullResponse.push_str(&Data.response);
				
				if Data.done {
					break;
				}
			}
		}
	}

	Ok(FullResponse)
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
