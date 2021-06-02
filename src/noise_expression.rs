use serde::{Deserialize,Serialize};
use rayon::prelude::*;
use std::collections::HashMap;
use std::borrow::Cow;

use crate::noise_generator::*;

#[derive(Serialize,Deserialize)]
pub struct NoiseMap{
    pub noise_dictionary: HashMap<NoiseTag, Box<dyn NoiseGenerator>>,
    pub generation_expression: GenerationExpressionToken
}


impl NoiseMap{
    pub fn generate_noise_map(&self, x_offset:f32, y_offset:f32, width: usize, height: usize) -> Vec<f32> {
        let compound_noise_map = self.build_compound_noise(x_offset, y_offset, width, height);
        let vec = self.generation_expression.get_vec(&compound_noise_map);
        Self::normalize(vec)
    }

    fn build_compound_noise(&self, x_offset:f32, y_offset:f32, width: usize, height: usize) -> HashMap<&NoiseTag, Vec<f32>>{
        self.noise_dictionary.par_iter().map(|(t,v)|{
            (t,v.get_noise(x_offset, y_offset, width, height))
        }).collect()
    }

    fn normalize(vec: Cow<Vec<f32>>) -> Vec<f32>{
        let min = vec.iter().fold(f32::NAN, |a, b| a.min(*b));
        let max = vec.iter().fold(f32::NAN, |a, b| a.max(*b));

        vec.iter().map(|f| (f-min)/(max-min)).collect()
    }
}

#[typetag::serde]
pub trait GenerationExpressionOperator{
    fn result(&self,available_noises: &HashMap<&NoiseTag, Vec<f32>>) -> Vec<f32>;
}

#[derive(Serialize,Deserialize)]
pub enum GenerationExpressionToken{
    Operator(Box<dyn GenerationExpressionOperator>),
    Noise(NoiseTag)
}

impl GenerationExpressionToken{
    pub fn get_vec<'a>(&self, available_noises: &'a HashMap<&NoiseTag, Vec<f32>>) -> Cow<'a,Vec<f32>> {
        match self{
            GenerationExpressionToken::Operator(operator) => {
                Cow::Owned(operator.result(available_noises))
            }
            GenerationExpressionToken::Noise(noise_tag) => {
                Cow::Borrowed(available_noises.get(noise_tag).unwrap())
            }
        }
    }
}