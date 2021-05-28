use std::collections::HashMap;
use std::array::IntoIter;
use std::iter::FromIterator;

use std::{ops::Range};
use rayon::prelude::*;
use time::PreciseTime;

use serde::{Deserialize,Serialize};
use simdnoise::NoiseBuilder;

use super::*;

#[derive(Serialize,Deserialize)]
struct Add{
    lhs: GenerationExpressionToken,
    rhs: GenerationExpressionToken,
}

#[typetag::serde]
impl GenerationExpressionOperator for Add{
    fn result(&self, available_noises: &HashMap<&NoiseTag, Vec<f32>>) -> Vec<f32> {
        let lhs_vec = self.lhs.get_vec(available_noises);
        let rhs_vec = self.rhs.get_vec(available_noises);
        let result: Vec<f32> = lhs_vec.par_iter().zip(rhs_vec.par_iter()).map(|(x,y)| x+y).collect();
        result
    }   
}


#[derive(Serialize,Deserialize)]
struct Mult{
    lhs: GenerationExpressionToken,
    rhs: GenerationExpressionToken,
}

#[typetag::serde]
impl GenerationExpressionOperator for Mult{
    fn result(&self, available_noises: &HashMap<&NoiseTag, Vec<f32>>) -> Vec<f32> {
        let lhs_vec = self.lhs.get_vec(available_noises);
        let rhs_vec = self.rhs.get_vec(available_noises);
        let result: Vec<f32> = lhs_vec.par_iter().zip(rhs_vec.par_iter()).map(|(x,y)| x*y).collect();
        result
    }   
}

#[derive(Serialize,Deserialize)]
struct PerlinNoiseConfig{
    freq: f32,
}
#[typetag::serde]
impl NoiseGenerator for PerlinNoiseConfig{
    fn get_noise(&self, x_offset:f32, y_offset:f32, width: usize, height: usize) -> Vec<f32>{
        NoiseBuilder::gradient_2d_offset(x_offset, width, y_offset, height)
            .with_freq(self.freq)
            .generate_scaled(0., 1.)
    }
}

#[derive(Serialize,Deserialize)]
struct UniformNoiseConfig{
    val: f32,
}
#[typetag::serde]
impl NoiseGenerator for UniformNoiseConfig{
    fn get_noise(&self, x_offset:f32, y_offset:f32, width: usize, height: usize) -> Vec<f32>{
        vec![self.val;width*height]
    }

}

#[cfg(test)]

    

    #[test]
fn deserialization(){
    let serialized_region_noise_config = r"---
noise_dictionary:
  A:
    PerlinNoiseConfig:
      freq: 0.0002
  B:
    PerlinNoiseConfig:
      freq: 0.01
  C:
    UniformNoiseConfig:
      val: 30.0
  D:
    PerlinNoiseConfig:
      freq: 0.001
generation_expression:
  Operator:
    Add:
      lhs:
        Operator:
          Mult:
            lhs:
              Noise: A
            rhs:
              Noise: C
      rhs:
        Operator: 
          Add:
            lhs:
              Noise: B
            rhs:
              Noise: D";

    let deserialized: ComplexNoise  = serde_yaml::from_str(&serialized_region_noise_config).unwrap();
}

#[test]
fn generation(){
    let region_noise_config = ComplexNoise{
        noise_dictionary: HashMap::from_iter(IntoIter::new([
            (NoiseTag("A".to_string()), Box::new(PerlinNoiseConfig{freq:0.001}) as Box<dyn NoiseGenerator>),
            (NoiseTag("B".to_string()), Box::new(PerlinNoiseConfig{freq:0.02}) as Box<dyn NoiseGenerator>),
            (NoiseTag("C".to_string()), Box::new(UniformNoiseConfig{val:5.}) as Box<dyn NoiseGenerator>),
        ])),
        generation_expression: GenerationExpressionToken::Operator(
            Box::new(Add{
                lhs: GenerationExpressionToken::Operator(
                    Box::new(Mult{
                        lhs: GenerationExpressionToken::Noise(NoiseTag("A".to_string())),
                        rhs: GenerationExpressionToken::Noise(NoiseTag("C".to_string()))
                    })),
                rhs: GenerationExpressionToken::Noise(NoiseTag("B".to_string()))
        }))
    };
    
    let result = region_noise_config.generate_noise(0., 0., 100, 100);
}