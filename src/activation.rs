

pub enum Activation {
    Sigmoid,
    TanH,
    ReLU,
    SiLU,
    SoftMax
}


impl Activation {
    pub fn activate(self, inputs: &[f32], index: usize) -> f32 {
        use Activation::*;
        match self {
            Sigmoid => sigmoid_activate(inputs, index),
            TanH => tanh_activate(inputs, index),
            ReLU => relu_activate(inputs, index),
            SiLU => silu_activate(inputs, index),
            SoftMax => softmax_activate(inputs, index)
        }
    }

    pub fn derivitave(self, inputs: &[f32], index: usize) -> f32 {
        use Activation::*;
        match self {
            Sigmoid => sigmoid_derivitave(inputs, index),
            TanH => tanh_derivitave(inputs, index),
            ReLU => relu_activate(inputs, index),
            SiLU => silu_derivitave(inputs, index),
            SoftMax => softmax_derivitave(inputs, index)
        }
    }

}



//Sigmoid
pub fn sigmoid_activate(inputs: &[f32], index: usize) -> f32 {
    1.0 / (1.0 + f32::exp(-inputs[index]))
}



pub fn sigmoid_derivitave(inputs: &[f32], index: usize) -> f32 {
    let a = sigmoid_activate(inputs, index);
    a * (1.0 - a)
}


//TanH
pub fn tanh_activate(inputs: &[f32], index: usize) -> f32 {
    let e2 = f32::exp(2.0 * inputs[index]);
    (e2 - 1.0) / (e2 + 1.0)
}

pub fn tanh_derivitave(inputs: &[f32], index: usize) -> f32 {
    let e2 = f32::exp(2.0 * inputs[index]);
    let t = (e2 - 1.0) / (e2 + 1.0);
    1.0 - t * t
}

//ReLu
pub fn relu_activate(inputs: &[f32], index: usize) -> f32 {
    f32::max(0.0, inputs[index])
}

pub fn relu_derivitave(inputs: &[f32], index: usize) -> f32 {
    ((inputs[index] > 0.0) as i32) as f32
}

//SiLu
pub fn silu_activate(inputs: &[f32], index: usize) -> f32 {
    inputs[index] / (1.0 + f32::exp(-inputs[index]))
}

pub fn silu_derivitave(inputs: &[f32], index: usize) -> f32 {
    let sig = 1.0 / (1.0 + f32::exp(-inputs[index]));
    inputs[index] * sig * (1.0 - sig) + sig
}

//SoftMax
pub fn softmax_activate(inputs: &[f32], index: usize) -> f32 {
    let exp_sum: f32 = inputs.iter().sum();
    f32::exp(inputs[index] / exp_sum)
}

pub fn softmax_derivitave(inputs: &[f32], index: usize) -> f32 {
    let exp_sum: f32 = inputs.iter().sum();
    let exp = f32::exp(inputs[index]);
    (exp * exp_sum - exp * exp) / (exp_sum * exp_sum)
} 
