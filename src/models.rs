use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub monitor_to_capture: usize,
    pub api_base: String,
    pub api_key: String,
    pub model: String,
    pub notification_sound_path: String,
    pub send_notification_overtime: bool,
    pub ignore_if_time_remaining_higher_than: f64,
}

#[derive(Serialize, Debug)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
}

#[derive(Serialize, Debug)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub content_type: String,
    pub json_schema: OuterSchema,
}

#[derive(Serialize, Debug)]
pub struct OuterSchema {
    pub name: String,
    pub strict: bool,
    pub schema: Value,
}

#[derive(Serialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: Vec<MessageContent>,
}

#[derive(Serialize, Debug)]
pub struct MessageContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
    pub image_url: Option<ImageObject>,
}

#[derive(Serialize, Debug)]
pub struct ImageObject {
    pub url: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct OpenAIResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
    pub stats: Option<serde_json::Value>,
    pub system_fingerprint: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub index: u32,
    pub message: AssistantMessage,
    pub finish_reason: Option<String>,
    pub logprobs: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct AssistantMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct ModelResponse {
    pub image_comparison_and_analysis: String,
    pub images_are_relevant: bool,
    pub reasoning: String,
    pub time_remaining_in_seconds: Option<f64>,
}
