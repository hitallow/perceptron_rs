pub mod perceptron {

    use crate::perceptron::{
        activation_functions::activation_functions::step_function,
        functions::functions::{calc_loss, sum_weights_and_inputs},
        sample::sample::Sample,
    };
    use std::mem;

    pub struct Perceptron {
        size_inputs: u32,
        samples: Vec<Sample>,
        weights: Vec<f32>,
        activation_function: fn(f32) -> f32,
        learning_rate: f32,
        max_epics: u32,
        accuracy: f32,
    }

    impl Perceptron {
        /*
         *  creat a perceptron instance
         */
        pub fn new(size_inputs: u32, max_epics: u32, learning_rate: f32) -> Perceptron {
            return Perceptron {
                size_inputs,
                samples: Vec::new(),
                weights: Vec::new(),
                activation_function: step_function,
                learning_rate,
                max_epics,
                accuracy: 0.0,
            };
        }

        pub fn push_sample(&mut self, sample: Sample) {
            if self.size_inputs != sample.get_inputs().len() as u32 {
                println!(
                    "The size of inputs of sample must be equal to {}",
                    self.size_inputs
                );
            } else {
                self.samples.push(sample);
            }
        }

        pub fn get_accuracy(&self) -> &f32 {
            return &self.accuracy;
        }

        /// Set the perceptron's weights.
        pub fn set_weights(&mut self, weights: &[f32]) {
            if self.size_inputs != weights.len() as u32 {
                println!("The size of weights must be equal to {}", self.size_inputs)
            } else {
                self.weights = weights.to_vec();
            }
        }

        pub fn get_weights(&self) -> &Vec<f32> {
            return &self.weights;
        }

        pub fn train(&mut self) {
            let samples = mem::replace(&mut self.samples, Vec::new());

            let mut prediction: f32;
            let mut sum: f32;
            let mut exists_miss = true;
            let mut current_epic = 0;
            let mut total_loss: u32 = 0;
            while exists_miss && current_epic < self.max_epics {
                total_loss = 0;
                exists_miss = false;

                for sample in &samples {
                    sum = sum_weights_and_inputs(&self.weights, sample.get_inputs());

                    prediction = (self.activation_function)(sum);

                    if prediction != *sample.get_target() {
                        exists_miss = true;
                        total_loss += 1;

                        for index in 0..self.weights.len() {
                            self.weights[index] = calc_loss(
                                sample.get_target(),
                                &(prediction as f32),
                                &(self.learning_rate),
                                &(self.weights[index]),
                                &sample.get_inputs()[index],
                            );
                        }
                    }
                }
                current_epic += 1;
            }

            self.accuracy = 1.0 - total_loss as f32 / samples.len() as f32;
        }
    }
}
