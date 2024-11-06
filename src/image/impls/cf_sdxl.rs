use std::time::Duration;

use reqwest::Client;
use serde::Serialize;

use crate::image::interface::GenerateImageStrategy;
use crate::models::{InputData, OutputData, OutputErrorData};

pub struct CFSdxlProcessor {
    client: Client,
}

impl CFSdxlProcessor {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[derive(Serialize)]
struct SdxlRequest {
    prompt: String,
    num_steps: i32,
}

#[async_trait::async_trait]
impl GenerateImageStrategy for CFSdxlProcessor {
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

        let request = SdxlRequest {
            prompt: input.prompt.clone(),
            num_steps: input.steps as i32,
        };

        let url = format!("https://gateway.ai.cloudflare.com/v1/{}/{}/workers-ai/@cf/bytedance/stable-diffusion-xl-lightning", account_id, gateway_name);

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

        if content_type != "image/png" {
            return Err(OutputErrorData::new("Invalid content type".to_string()));
        }
        let image_data = match response.bytes().await {
            Ok(bytes) => bytes.to_vec(),
            Err(e) => {
                return Err(OutputErrorData::new(format!(
                    "Failed to get image data: {}",
                    e
                )));
            }
        };
        Ok(OutputData::new(image_data.to_vec()))
    }
}
