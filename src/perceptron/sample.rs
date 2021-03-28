pub mod sample {
    pub struct Sample {
        inputs: Vec<f32>,
        target: f32,
    }

    impl Sample {
        pub fn new_sample(inputs: Vec<f32>, target: f32) -> Sample {
            return Sample { inputs, target };
        }

        pub fn get_inputs(&self) -> &Vec<f32> {
            return &self.inputs;
        }

        pub fn get_target(&self) -> &f32 {
            return &self.target;
        }
    }
}
