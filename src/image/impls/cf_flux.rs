use std::time::Duration;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::image::interface::GenerateImageStrategy;
use crate::models::{InputData, OutputData, OutputErrorData};

pub struct CFFluxProcessor {
    client: Client,
}

impl CFFluxProcessor {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[derive(Serialize)]
struct FluxRequest {
    prompt: String,
    num_steps: i32,
}

#[derive(Deserialize)]
struct FluxResponse {
    result: ResultData,
}

#[derive(Deserialize)]
struct ResultData {
    image: String, // pngをbase64エンコードした文字列
}

#[async_trait::async_trait]
impl GenerateImageStrategy for CFFluxProcessor {
    async fn process(&self, input: &InputData) -> Result<OutputData, OutputErrorData> {
        let account_id = match self.get_env("CLOUDFLARE_ACCOUNT_ID") {
            Some(account_id) => account_id,
            None => {
                return Err(OutputErrorData::new(
                    "$CLOUDFLARE_ACCOUNT_ID must be set".to_string(),
                ));
            }
        };
        let api_token = match self.get_env("CLOUDFLARE_API_TOKEN") {
            Some(api_token) => api_token,
            None => {
                return Err(OutputErrorData::new(
                    "$CLOUDFLARE_API_TOKEN must be set".to_string(),
                ));
            }
        };
        let gateway_name = match self.get_env("CLOUDFLARE_GATEWAY_NAME") {
            Some(gateway_name) => gateway_name,
            None => {
                return Err(OutputErrorData::new(
                    "$CLOUDFLARE_GATEWAY_NAME must be set".to_string(),
                ));
            }
        };

        let request = FluxRequest {
            prompt: input.prompt.clone(),
            num_steps: input.steps as i32,
        };

        let url = format!("https://gateway.ai.cloudflare.com/v1/{}/{}/workers-ai/@cf/black-forest-labs/flux-1-schnell", account_id, gateway_name);

        let response = match self
            .client
            .post(&url)
            .timeout(Duration::from_secs(30))
            .header("Authorization", format!("Bearer {}", api_token))
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

        let image_data = match BASE64_STANDARD.decode(response.result.image) {
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
