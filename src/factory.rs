use crate::image::impls::{CFFluxProcessor, CFSdxlProcessor, TAFluxProcessor};
use crate::image::interface::GenerateImageStrategy;

pub struct ProcessorFactory;

impl ProcessorFactory {
    pub fn create(type_name: &str) -> Option<Box<dyn GenerateImageStrategy>> {
        match type_name {
            "cf_flux" => Some(Box::new(CFFluxProcessor::new())),
            "cf_sdxl" => Some(Box::new(CFSdxlProcessor::new())),
            "ta_flux" => Some(Box::new(TAFluxProcessor::new())),
            _ => None,
        }
    }
}
