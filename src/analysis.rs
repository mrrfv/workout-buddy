use crate::models::*;
use serde_json::json;
use xcap::Monitor;
use base64::Engine;
use std::io::Cursor;

pub async fn run_analysis(config: &AppConfig) -> Result<(ModelResponse, Option<f64>), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let monitor_num = config.monitor_to_capture;

    // Take 3 screenshots separated by 1 second, save them into a vector
    let mut screenshots = Vec::new();
    let mut start_time = std::time::Instant::now();
    for i in 0..3 {
        println!("Taking screenshot {}...", i + 1);
        if i == 2 {
            // Record the current time, in order to compensate for the model's latency
            start_time = std::time::Instant::now();
        }
        let screenshot = take_base64_screenshot(monitor_num)?;
        screenshots.push(screenshot);
        if i < 2 {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    // Create a task string
    let task = "Perform the analysis in accordance with the system prompt.".to_string();

    // Load the system prompt from a file "system_prompt.txt"
    let system_prompt = include_str!("system_prompt.txt").to_string();

    println!("Sending request to LLM...");

    // Create the OpenAI request
    let resp = create_openai_request(
        client.clone(),
        &config.clone(),
        screenshots,
        task,
        system_prompt,
        json!({
            "type": "object",
            "description": "Schema for analyzing images and identifying a countdown in a video.",
            "properties": {
                "image_comparison_and_analysis": {
                    "type": "string",
                    "description": "A thorough comparison and analysis of each image, looking for patterns and clues as to where the countdown is in the video, if any. In most cases, the countdown is a number in the video that's decreasing, since the images are taken within 2 seconds of each other.",
                    "minLength": 100
                },
                "images_are_relevant": {
                    "type": "boolean",
                    "description": "Whether the images contain a playing video along with a countdown present. This should be true only if the requirements are met. For example, a screenshot of the comments section or a messaging app is irrelevant.",
                    "default": false
                },
                "reasoning": {
                    "type": "string",
                    "description": "Reasoning behind the choice of the countdown. This is also a place to verify your choice.",
                    "minLength": 5
                },
                "time_remaining_in_seconds": {
                    "type": "number",
                    "description": "The time remaining of the current workout element.",
                    "minimum": 0
                }
            },
            "required": [
                "image_comparison_and_analysis",
                "images_are_relevant",
                "reasoning"
            ]
        }).into()
    ).await?;

    // Calculate the time taken for the model to respond
    let elapsed_time = start_time.elapsed();

    println!("Model response received in {:.2} seconds", elapsed_time.as_secs_f64());

    // Process the response
    let model_response: ModelResponse = serde_json::from_str(&resp.choices[0].message.content)?;

    // Compensate for the model's latency by subtracting the elapsed time to the time remaining
    let compensated_time = model_response
        .time_remaining_in_seconds
        .map(|time_remaining| time_remaining - elapsed_time.as_secs_f64());

    Ok((model_response, compensated_time))
}

fn take_base64_screenshot(monitor_num: usize) -> Result<String, Box<dyn std::error::Error>> {
    let monitor = &Monitor::all()?[monitor_num];
    let mut image_bytes: Vec<u8> = vec![];
    monitor.capture_image()?
        .write_to(&mut Cursor::new(&mut image_bytes), image::ImageFormat::Png)?;
    let base64_encoded_image = base64::engine::general_purpose::STANDARD.encode(&image_bytes);
    Ok(format!("data:image/png;base64,{}", base64_encoded_image))
}

async fn create_openai_request(
    client: reqwest::Client,
    config: &AppConfig,
    screenshots: Vec<String>,
    task: String,
    system_prompt: String,
    structured_output_schema: Option<serde_json::Value>,
) -> Result<OpenAIResponse, Box<dyn std::error::Error>> {
    // Map the screenshots into MessageContent objects, with a type of "image_url"
    let image_objects: Vec<MessageContent> = screenshots.into_iter().map(|screenshot| {
        MessageContent {
            content_type: "image_url".to_string(),
            text: None,
            image_url: Some(ImageObject { url: screenshot }),
        }
    }).collect();

    // Create the system message
    let system_message = Message {
        role: "system".to_string(),
        content: vec![MessageContent {
            content_type: "text".to_string(),
            text: Some(system_prompt),
            image_url: None,
        }],
    };

    // Create the user message with the task and the images
    let user_message = Message {
        role: "user".to_string(),
        content: vec![MessageContent {
            content_type: "text".to_string(),
            text: Some(task),
            image_url: None,
        }]
        .into_iter()
        .chain(image_objects.into_iter())
        .collect(),
    };

    // Compose the messages in order: system, then user
    let message = vec![system_message, user_message];

    // Create the OpenAI request
    let openai_request = OpenAIRequest {
        model: config.model.clone(),
        messages: message,
        response_format: Some(ResponseFormat {
            content_type: "json_schema".to_string(),
            json_schema: OuterSchema { name: "schema".to_string(), strict: true, schema: structured_output_schema.unwrap() }
        })
    };

    // Send the request to the API
    let resp = client
        .post(format!("{}/chat/completions", config.api_base))
        .bearer_auth(&config.api_key)
        .json(&openai_request)
        .send()
        .await?;

    if resp.status().is_success() {
        let response_json = resp.json::<OpenAIResponse>().await?;
        Ok(response_json)
    } else {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        Err(format!("API error: {} - {}", status, text).into())
    }
}
