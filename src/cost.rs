


pub enum Cost {
    MeanSquareError,
    CrossEntropy
}


impl Cost {
    pub fn cost(self, predicted_outputs: &[f32], expected_outputs: &[f32]) -> f32 {
        match self {
            Cost::MeanSquareError => {
                let mut cost = 0.0;
                for i in 0..predicted_outputs.len() {
                    let error  = predicted_outputs[i] - expected_outputs[i];
                    cost += error * error;
                }
                0.5 * cost
            },
            Cost::CrossEntropy => {
                let mut cost = 0.0;
                for i in 0..predicted_outputs.len() {
                    let pred = predicted_outputs[i];
                    let exp = expected_outputs[i];
                    let val = if exp == 1.0 { -f32::ln(pred) } else { -f32::ln(1.0 - pred) }; 
                    cost += if val.is_nan() { 0.0 } else { val };
                }
                cost
            }
        }
    }       
    
    pub fn cost_derivitave(self, predicted_output: f32, expected_output: f32) -> f32 {
        match self {
            Cost::MeanSquareError => predicted_output - expected_output,
            Cost::CrossEntropy => {
               if predicted_output == 0.0 || expected_output == 1.0 {
                    return 0.0; 
               }
               (-predicted_output + expected_output) / (predicted_output * (predicted_output - 1.0))
            },
        }
    }

}
