use crate::image::interface::GenerateImageStrategy;
use crate::models::{InputData, OutputData, OutputErrorData};

pub struct ProcessContext {
    strategy: Box<dyn GenerateImageStrategy>,
}

impl ProcessContext {
    pub fn new(strategy: Box<dyn GenerateImageStrategy>) -> Self {
        Self { strategy }
    }

    pub async fn execute(&self, input: InputData) -> Result<OutputData, OutputErrorData> {
        self.strategy.process(&input).await
    }
}
