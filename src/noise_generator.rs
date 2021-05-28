use serde::{Deserialize,Serialize};
use rayon::prelude::*;

#[derive(PartialEq,Eq,Hash,Serialize,Deserialize)]
pub struct NoiseTag(pub String);
#[typetag::serde]
pub trait NoiseGenerator: Sync + Send + 'static{
    fn get_noise(&self, x_offset:f32, y_offset:f32, width: usize, height: usize) -> Vec<f32>;
}