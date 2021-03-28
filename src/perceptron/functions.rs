pub mod functions {
    pub fn calc_loss(
        target: &f32,
        prediction: &f32,
        learning_rate: &f32,
        weight: &f32,
        input: &f32,
    ) -> f32 {
        let miss: f32 = target - prediction;
        let new_weight: f32 = weight + learning_rate * miss * input;
        return new_weight;
    }

    pub fn sum_weights_and_inputs(weights: &[f32], inputs: &[f32]) -> f32 {
        let mut summatory: f32 = 0.0;

        for index in 0..inputs.len() as usize {
            let weight = weights[index];
            let input = inputs[index];
            summatory += weight * input;
        }

        return summatory;
    }
}
