mod models;
mod utils;
mod analysis;

use std::time::Duration;

use models::*;
use utils::*;
use analysis::run_analysis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the configuration from a file "config.json" and parse it
    let config_data = std::fs::read_to_string("config.json").expect("Failed to read config file. Make sure it exists (config.json) and is valid.");
    let config: AppConfig = serde_json::from_str(&config_data)?;
    println!("Configuration loaded: {:#?}", config);

    println!("Starting in 5 seconds! Get ready!");
    tokio::time::sleep(Duration::from_secs(5)).await;

    loop {
        println!("Starting analysis...");
        let (model_response, compensated_time) = run_analysis(&config).await?;
        // Ignore if the images are not relevant
        if model_response.images_are_relevant {
            if let Some(time_remaining) = compensated_time {
                println!("Compensated Time Remaining: {:.2} seconds", time_remaining);

                // If the time remaining is higher than the user-provided threshold, assume it as invalid and carry on
                if time_remaining > config.ignore_if_time_remaining_higher_than {
                    println!(
                        "Time remaining ({:.2}) is higher than threshold ({:.2}), reprocessing.",
                        time_remaining, config.ignore_if_time_remaining_higher_than
                    );
                    continue;
                }
                if time_remaining <= 0.0 {
                    // This should happen only if the model is so slow that it takes longer than the time remaining to respond.
                    println!("The workout element has ended.");
                    if config.send_notification_overtime {
                        // Send the user a notification
                        notify_user("The workout element has ended, which the model didn't process on time.", &config.notification_sound_path).await?;
                    }
                } else {
                    println!("The workout element is still ongoing.");
                    // Sleep for the remaining time
                    tokio::time::sleep(tokio::time::Duration::from_secs_f64(time_remaining)).await;
                    // Send the user a notification
                    notify_user("The workout element has ended.", &config.notification_sound_path).await?;
                }
            } else {
                println!("No time remaining provided in the response.");
            }
        } else {
            println!("The images are not relevant for the analysis, according to the model: {:#?}", model_response);
        }
    }
}
