#![allow(unused_variables)]
use std::collections::HashMap;
use std::array::IntoIter;
use std::iter::FromIterator;

use rayon::prelude::*;
use std::path::Path;

use serde::{Deserialize,Serialize};
use simdnoise::{NoiseBuilder,CellDistanceFunction};

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
struct FBMNoiseConfig{
  freq: f32,
  octaves: u8,
  lacunarity: f32,
}
#[typetag::serde]
impl NoiseGenerator for FBMNoiseConfig{
  fn get_noise(&self, x_offset:f32, y_offset:f32, width: usize, height: usize) -> Vec<f32>{
      NoiseBuilder::fbm_2d_offset(x_offset, width, y_offset, height)
          .with_freq(self.freq)
          .with_octaves(self.octaves)
          .with_lacunarity(self.lacunarity)
          .generate_scaled(0., 1.)
  }
}

#[derive(Serialize,Deserialize)]
struct TurbNoiseConfig{
  freq: f32,
  octaves: u8,
  lacunarity: f32,
}
#[typetag::serde]
impl NoiseGenerator for TurbNoiseConfig{
  fn get_noise(&self, x_offset:f32, y_offset:f32, width: usize, height: usize) -> Vec<f32>{
      NoiseBuilder::turbulence_2d_offset(x_offset, width, y_offset, height)
          .with_freq(self.freq)
          .with_octaves(self.octaves)
          .with_lacunarity(self.lacunarity)
          .generate_scaled(0., 1.)
  }
}

#[derive(Serialize,Deserialize)]
struct CellConfig{
  freq: f32,

}
#[typetag::serde]
impl NoiseGenerator for CellConfig{
  fn get_noise(&self, x_offset:f32, y_offset:f32, width: usize, height: usize) -> Vec<f32>{
      NoiseBuilder::cellular2_2d_offset(x_offset, width, y_offset, height)
          .with_freq(self.freq)
          .with_distance_function(CellDistanceFunction::Manhattan)
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

    let deserialized: NoiseMap  = serde_yaml::from_str(&serialized_region_noise_config).unwrap();
}

#[test]
fn generation(){
    let region_noise_config = NoiseMap{
        noise_dictionary: HashMap::from_iter(IntoIter::new([
            (NoiseTag("A".to_string()), Box::new(PerlinNoiseConfig{freq:0.001}) as Box<dyn NoiseGenerator>),
            (NoiseTag("B".to_string()), Box::new(FBMNoiseConfig{freq:0.02, octaves: 3, lacunarity: 1.2}) as Box<dyn NoiseGenerator>),
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
    
    let result = region_noise_config.generate_noise_map(0., 0., 100, 100);
}

// #[test]
// fn generation_to_file(){
//     let region_noise_config = NoiseMap{
//         noise_dictionary: HashMap::from_iter(IntoIter::new([
//             (NoiseTag("A".to_string()), Box::new(CellConfig{freq:0.01}) as Box<dyn NoiseGenerator>),
//             (NoiseTag("B".to_string()), Box::new(FBMNoiseConfig{freq:0.002, octaves: 2, lacunarity: 0.5}) as Box<dyn NoiseGenerator>),
//             (NoiseTag("C".to_string()), Box::new(UniformNoiseConfig{val:0.}) as Box<dyn NoiseGenerator>),
//             (NoiseTag("D".to_string()), Box::new(UniformNoiseConfig{val:1.}) as Box<dyn NoiseGenerator>),
//         ])),
//         generation_expression: GenerationExpressionToken::Operator(
//             Box::new(Mult{
//                 lhs: GenerationExpressionToken::Noise(NoiseTag("A".to_string())),
//                 rhs: GenerationExpressionToken::Noise(NoiseTag("B".to_string())),
//         }))
//     };
    
//     let w = 1600;
//     let h = 1200;


//     let result = region_noise_config.generate_noise_map(0., 0., w, h);
//     let buffer: Vec<u8> = result.iter().map(|x| (x*255.) as u8).collect();
//     image::save_buffer(&Path::new("image.png"), &buffer, w as u32, h as u32, image::ColorType::L8).unwrap();
// }