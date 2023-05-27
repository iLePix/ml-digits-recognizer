use rand::{Rng, rngs::ThreadRng};

use crate::{IMAGE_HEIGHT, IMAGE_WIDTH};


pub struct Net {
    layers: Vec<Layer>,
}

impl Net {
    pub fn new(layer_sizes: &[usize]) -> Self {
        let mut rng = rand::thread_rng();
        let layers = layer_sizes.windows(2)
            .map(|sizes| Layer::new(sizes[0], sizes[1], &mut rng))
            .collect();
        Self { layers }
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
    axons: Vec<Axon>
}

impl Layer {
    pub fn new(in_nodes_num: usize, out_nodes_num: usize, rng: &mut ThreadRng) -> Self {
        let axons = (0.. in_nodes_num * out_nodes_num)
            .map(|_| Axon::from_rng(rng))
            .collect();
        Self {
            in_nodes_num,
            out_nodes_num,
            axons
       } 
    }
}
