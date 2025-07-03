use notify_rust::Notification;
use rodio::{OutputStream, Sink, Decoder};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub async fn notify_user(message: &str, notification_sound_path: &String) -> Result<(), Box<dyn std::error::Error>> {
    Notification::new()
        .summary("Workout Buddy")
        .body(message)
        .show()?;
    // Play a notification sound if available
    if play_audio_if_exists(&notification_sound_path) {
        println!("Playing notification sound: {}", &notification_sound_path);
    } else {
        println!("No notification sound played.");
    }
    Ok(())
}

pub fn play_audio_if_exists(file_path: &str) -> bool {
    if !Path::new(file_path).exists() {
        eprintln!("File does not exist: {}", file_path);
        return false;
    }

    let path = file_path.to_string();

    // Spawn background task with Tokio
    tokio::spawn(async move {
        // Run in a blocking task so rodio doesn't block async threads
        tokio::task::spawn_blocking(move || {
            let (_stream, stream_handle) =
                OutputStream::try_default().expect("Failed to get default output stream");
            let sink = Sink::try_new(&stream_handle).expect("Failed to create audio sink");

            let file = File::open(&path).expect("Failed to open file");
            let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");

            sink.append(source);

            // Block until playback finishes
            sink.sleep_until_end();
        })
        .await
        .expect("Blocking task panicked");
    });

    true
}
