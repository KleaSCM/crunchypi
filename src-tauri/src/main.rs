// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/**
 * Application Entry Point.
 * 
 * Invokes the library run function to start the Tauri application.
 * 
 * Author: KleaSCM
 * Email: KleaSCM@gmail.com
 */

fn main() {
	// Fix for "Failed to create GBM buffer" on some Linux setups (NVIDIA/VMs).
	// Disables hardware accelerated rendering for WebKit to prevent crashes.
	std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

	crunchypi_lib::run()
}
