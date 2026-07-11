use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};
use std::env;
use super::models::{CopilotRequest, AIResponsePayload};

const SYSTEM_PROMPT: &str = r#"
You are the AI Co-Pilot for 'Redner Studio', a professional video editing engine.
You will receive a JSON snapshot of the user's timeline (tracks, clips, playhead position).
You will also receive a user prompt asking you to perform an action.

Your goal is to interpret the user's intent and execute the correct timeline operations.

You MUST respond in strictly valid JSON format matching this exact schema:
{
  "thoughts": "A short, cool, one-sentence explanation of what you are doing (e.g., 'Moving playhead to 5s').",
  "operations": [
    { "type": "PLAY_PAUSE" },
    { "type": "SEEK", "timeMs": 5000.0 },
    { "type": "DELETE_CLIP", "clipId": "clip-uuid-here" },
    { "type": "SPLIT_CLIP", "clipId": "clip-uuid-here", "timeMs": 5000.0 }
  ]
}

RULES:
1. NEVER wrap the response in markdown blocks (no ```json). Return ONLY the raw JSON object.
2. If the user asks to "jump to 5 seconds", convert it to milliseconds (5000.0) for the SEEK operation.
3. If the user asks to "cut the selected clip", check the context for the `selectedClipId`. If none is selected, explain in "thoughts" that a clip needs to be selected and return empty operations.
4. If a prompt is ambiguous, perform the safest logical operation.
"#;

pub async fn copilot_chat(
    Json(raw_payload): Json<Value>,
) -> impl IntoResponse {

    let payload: CopilotRequest = match serde_json::from_value(raw_payload.clone()) {
        Ok(p) => p,
        Err(e) => {
            println!("🚨 422 ERROR: Invalid Frontend Data: {}", e);
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(json!({ "error": e.to_string() }))).into_response();
        }
    };

    println!("🤖 AI REQUEST RECEIVED: {}", payload.prompt);

    // 1. Clean the API key from the environment
    let raw_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set in .env");
    let api_key = raw_key.replace('"', "").trim().to_string();

    let context_str = serde_json::to_string_pretty(&payload.context).unwrap_or_default();
    let user_instruction = format!("Timeline Context:\n{}\n\nUser Prompt: {}", context_str, payload.prompt);

    let gemini_req = json!({
        "systemInstruction": { "parts": [{"text": SYSTEM_PROMPT}] },
        "contents": [{ "parts": [{"text": user_instruction}] }],
        "generationConfig": { "responseMimeType": "application/json" }
    });

    // 2. Trick the Clipboard Markdown Bug and USE THE ACTIVE 2026 MODEL (Gemini 3.5 Flash)
    let parsed_url = reqwest::Url::parse(concat!(
        "ht", "tps", "://", 
        "generativelanguage", 
        ".googleapis.com", 
        "/v1beta/models/", 
        "gemini-3.5-flash:generateContent"
    ))
    .expect("Hardcoded Gemini URL must always be valid");

    // 3. Build the HTTP Client
    let client = match reqwest::Client::builder().no_proxy().build() {
        Ok(c) => c,
        Err(e) => {
            println!("🚨 TLS/CLIENT BUILDER ERROR: {:#?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Backend HTTP client failed to build" }))).into_response();
        }
    };

    // 4. Send the Request using the clean header and the pristine API key string
    let res = match client.post(parsed_url)
        .header("x-goog-api-key", &api_key)
        .json(&gemini_req)
        .send()
        .await {
        Ok(response) => {
            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                println!("🚨 GOOGLE API REJECTED REQUEST: HTTP {} - {}", status, error_text);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Google API rejected the request" }))).into_response();
            }
            response
        },
        Err(e) => {
            println!("🚨 GEMINI NETWORK ERROR: {:#?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to connect to Google API" }))).into_response();
        }
    };

    let gemini_json: Value = res.json().await.unwrap_or_default();

    let ai_response_text = gemini_json
        .pointer("/candidates/0/content/parts/0/text")
        .and_then(|v| v.as_str())
        .unwrap_or("{}");

    // 5. THE BULLETPROOF JSON EXTRACTOR
    let start = ai_response_text.find('{');
    let end = ai_response_text.rfind('}');
    
    let clean_json = match (start, end) {
        (Some(s), Some(e)) if s <= e => &ai_response_text[s..=e],
        _ => "{}",
    };

    let final_payload: AIResponsePayload = match serde_json::from_str(clean_json) {
        Ok(p) => p,
        Err(e) => {
            println!("🚨 LLM SCHEMA MISMATCH: {}", e);
            println!("🚨 Raw LLM Output: {}", ai_response_text);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "AI returned invalid commands" }))).into_response();
        }
    };

    println!("✅ COMMANDS DISPATCHED TO UI: {:?}", final_payload.operations);

    Json(final_payload).into_response()
}