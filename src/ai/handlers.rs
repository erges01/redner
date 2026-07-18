use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::{json, Value};
use std::env;
use super::models::{CopilotRequest, AIResponsePayload};

const SYSTEM_PROMPT: &str = r#"
You are the AI Co-Pilot for 'Redner Studio', a professional video editing engine.
You will receive a JSON snapshot of the user's timeline (tracks, clips, playhead position).
You will also see the conversation history and the latest user prompt.

Your goal is to interpret intent and execute timeline operations or answer queries.

You MUST respond in strictly valid JSON format matching this exact schema:
{
  "thoughts": "A short, cool explanation of your response or action.",
  "operations": [
    { "type": "PLAY_PAUSE" },
    { "type": "SEEK", "timeMs": 5000.0 },
    { "type": "DELETE_CLIP", "clipId": "clip-uuid" },
    { "type": "SPLIT_CLIP", "clipId": "clip-uuid", "timeMs": 5000.0 },
    { "type": "MOVE_CLIP", "clipId": "clip-uuid", "newStartMs": 10000.0 },
    { "type": "DUPLICATE_CLIP", "clipId": "clip-uuid" },
    { "type": "CREATE_MARKER", "timeMs": 5000.0, "label": "Beat Drop" }
  ]
}

RULES:
1. NEVER wrap the response in markdown blocks (no ```json). Return ONLY the raw JSON object.
2. If the user asks a casual question, leave the "operations" array empty [].
3. For SEEK, SPLIT_CLIP, MOVE_CLIP, or CREATE_MARKER, convert seconds to milliseconds (e.g., 5s = 5000.0).
4. If asked to act on a clip but none is selected, check the timeline context. If ambiguous, explain in "thoughts" and return [].
"#;

pub async fn copilot_chat(
    Json(payload): Json<CopilotRequest>,
) -> impl IntoResponse {

    println!("🤖 AI REQUEST RECEIVED: {}", payload.prompt);

    // 1. Clean the API key from the environment
    let raw_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set in .env");
    let api_key = raw_key.replace('"', "").trim().to_string();

    // 2. Format the full chat history for Gemini's structural payload
    let mut gemini_contents = Vec::new();

    // Append prior conversation turns
    for msg in payload.history {
        let role_string = if msg.role == "model" { "model" } else { "user" };
        gemini_contents.push(json!({
            "role": role_string,
            "parts": [{ "text": msg.text }]
        }));
    }

    // Append current turn along with the normalized timeline context injection
    let context_str = serde_json::to_string_pretty(&payload.context).unwrap_or_default();
    let final_user_turn = format!("Timeline Context:\n{}\n\nUser Prompt: {}", context_str, payload.prompt);
    
    gemini_contents.push(json!({
        "role": "user",
        "parts": [{ "text": final_user_turn }]
    }));

    let gemini_req = json!({
        "systemInstruction": { "parts": [{"text": SYSTEM_PROMPT}] },
        "contents": gemini_contents,
        "generationConfig": { "responseMimeType": "application/json" }
    });

    // 3. Trick the Clipboard Markdown Bug and USE THE ACTIVE 2026 MODEL (Gemini 3.5 Flash)
    let parsed_url = reqwest::Url::parse(concat!(
        "ht", "tps", "://", 
        "generativelanguage", 
        ".googleapis.com", 
        "/v1beta/models/", 
        "gemini-3.5-flash:generateContent"
    ))
    .expect("Hardcoded Gemini URL must always be valid");

    // 4. Build the HTTP Client
    let client = match reqwest::Client::builder().no_proxy().build() {
        Ok(c) => c,
        Err(e) => {
            println!("🚨 TLS/CLIENT BUILDER ERROR: {:#?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Backend HTTP client failed to build" }))).into_response();
        }
    };

    // 5. Send the Request with an Automatic Retry Loop for 503s
    let mut attempt = 0;
    let max_retries = 3;
    let mut response_text = String::new();

    loop {
        attempt += 1;
        match client.post(parsed_url.clone())
            .header("x-goog-api-key", &api_key)
            .json(&gemini_req)
            .send()
            .await 
        {
            Ok(response) => {
                let status = response.status();
                let text = response.text().await.unwrap_or_default();

                if status == StatusCode::SERVICE_UNAVAILABLE && attempt < max_retries {
                    println!("⚠️ GOOGLE API 503 OVERLOAD. Retrying {}/{} in 2s...", attempt, max_retries);
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    continue;
                } else if !status.is_success() {
                    println!("🚨 GOOGLE API REJECTED REQUEST: HTTP {} - {}", status, text);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Google API rejected the request" }))).into_response();
                }
                
                response_text = text;
                break;
            },
            Err(e) => {
                println!("🚨 GEMINI NETWORK ERROR: {:#?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to connect to Google API" }))).into_response();
            }
        }
    }

    let gemini_json: Value = serde_json::from_str(&response_text).unwrap_or_default();

    let ai_response_text = gemini_json
        .pointer("/candidates/0/content/parts/0/text")
        .and_then(|v| v.as_str())
        .unwrap_or("{}");

    // 6. THE BULLETPROOF JSON EXTRACTOR
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