use std::time::Duration;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::image::interface::GenerateImageStrategy;
use crate::models::{InputData, OutputData, OutputErrorData};

pub struct TAFluxProcessor {
    client: Client,
}

impl TAFluxProcessor {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[derive(Serialize)]
struct FluxRequest {
    model: String, // "black-forest-labs/FLUX.1-schnell-Free"
    prompt: String,
    width: i32, // 1024
    height: i32, // 1024
    steps: i32, // 1~4
    n: i32, // 1
    response_format: String, // "base64"
    guidance: f32, // 1.0~10.0, default 3.5
}

impl FluxRequest {
    pub fn new(prompt: String, steps: i32) -> Self {
        Self {
            model: "black-forest-labs/FLUX.1-schnell-Free".to_string(),
            prompt,
            width: 1024,
            height: 1024,
            steps,
            n: 1,
            response_format: "base64".to_string(),
            guidance: 3.5,
        }
    }
}

#[derive(Deserialize)]
struct FluxResponse {
    id: String,
    model: String, // "black-forest-labs/FLUX.1-schnell-Free"
    object: String,
    data: Vec<ResultDataItem>,
}

#[derive(Deserialize)]
struct ResultDataItem {
    timings: ResultTimings,
    index: u32,
    b64_json: String, // jpegをbase64エンコードした文字列
}

#[derive(Deserialize)]
struct ResultTimings {
    inference: f64,
}

#[async_trait::async_trait]
impl GenerateImageStrategy for TAFluxProcessor {
    async fn process(&self, input: &InputData) -> Result<OutputData, OutputErrorData> {
        if input.steps < 1 || input.steps > 4 {
            return Err(OutputErrorData::new("Invalid steps".to_string()));
        }

        let api_token = match self.get_env("TOGETHERAI_API_TOKEN") {
            Some(api_token) => api_token,
            None => {
                return Err(OutputErrorData::new(
                    "$TOGETHERAI_API_TOKEN must be set".to_string(),
                ));
            }
        };

        let request = FluxRequest::new(input.prompt.clone(), input.steps as i32);

        let url = "https://api.together.xyz/v1/images/generations";

        let response = match self
            .client
            .post(url)
            .timeout(Duration::from_secs(30))
            .header("Authorization", format!("Bearer {}", api_token))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&request)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                return Err(OutputErrorData::new(format!(
                    "Failed to send request: {}",
                    e
                )));
            }
        };
        let response = match response.error_for_status() {
            Ok(resp) => resp,
            Err(e) => {
                return Err(OutputErrorData::new(format!(
                    "Failed to get response: {}",
                    e
                )));
            }
        };

        let content_type = match response.headers().get("Content-Type") {
            Some(content_type) => content_type,
            None => {
                return Err(OutputErrorData::new("Content type not found".to_string()));
            }
        };

        if content_type != "application/json" {
            return Err(OutputErrorData::new("Invalid content type".to_string()));
        }

        //serde_json
        let response = match response.json::<FluxResponse>().await {
            Ok(resp) => resp,
            Err(e) => {
                return Err(OutputErrorData::new(format!(
                    "Failed to parse response: {}",
                    e
                )));
            }
        };

        if response.data.is_empty() {
            return Err(OutputErrorData::new("Image data not found".to_string()));
        }

        let image_data = match BASE64_STANDARD.decode(&response.data[0].b64_json) {
            Ok(bytes) => bytes,
            Err(e) => {
                return Err(OutputErrorData::new(format!(
                    "Failed to decode image data: {}",
                    e
                )));
            }
        };
        Ok(OutputData::new(image_data.to_vec()))
    }
}
