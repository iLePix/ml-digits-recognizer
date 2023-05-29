use std::array;

use rand::{Rng, rngs::ThreadRng};

use crate::{IMAGE_HEIGHT, IMAGE_WIDTH, activation::{Activation, self}};


pub struct Net {
    activation: Activation,
    layers: Vec<Layer>,
}

impl Net {
    pub fn new(layer_sizes: &[usize], activation: Activation) -> Self {
        let mut rng = rand::thread_rng();
        let layers = layer_sizes.windows(2)
            .map(|sizes| Layer::new(sizes[0], sizes[1], &mut rng))
            .collect();
        Self { layers, activation }
    }
    
    //running the inputs through the neural net and returning the outputs
    pub fn think(inputs: &[f32]) -> &[f32] {
        inputs
    }
}


#[derive(Clone)]
pub struct Axon {
    pub weight: f32,
    pub bias: f32
}

impl Axon {
    pub fn zero() -> Self {
      Self { weight: 0.0, bias: 0.0 }
    }

    pub fn new(weight: f32, bias: f32) -> Self {
        Self { weight, bias }
    }

    pub fn from_rng(rng: &mut ThreadRng) -> Self {
        let weight = rng.gen_range(0.0..=1.0);
        let bias = rng.gen_range(0.0..=1.0);
        Self { weight, bias }
    }
}

#[derive(Clone)]
pub struct Layer {
    out_nodes_num: usize,
    in_nodes_num: usize,
    weights: Vec<f32>,
    biases: Vec<f32>,
}

impl Layer {
    pub fn new(in_nodes_num: usize, out_nodes_num: usize, rng: &mut ThreadRng) -> Self {
        let weights = (0..in_nodes_num * out_nodes_num)
            .map(|_| rng.gen_range(0.0..=1.0)) 
            .collect();
        
        let biases = (0..in_nodes_num * out_nodes_num)
            .map(|_| rng.gen_range(0.0..=1.0)) 
            .collect();
        Self {
            in_nodes_num,
            out_nodes_num,
            weights,
            biases
       } 
    }

    fn get_weight(&self, in_node: usize, out_node: usize) -> f32 {
        self.weights[out_node * self.in_nodes_num + in_node]
    }

    pub fn calculate_outputs(&self, inputs: &[f32]) -> Vec<f32> {
        let outputs = Vec::with_capacity(self.out_nodes_num);
        for out_node in 0..self.out_nodes_num {
            let mut weighted_output = self.biases[out_node];
            for in_node in 0..self.in_nodes_num {
               weighted_output += self.get_weight(in_node, out_node) * inputs[in_node];  
            }
        }
        outputs
    }
}
