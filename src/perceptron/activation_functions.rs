pub mod activation_functions {
    pub fn step_function(value: f32) -> f32 {
        if value >= 0.0 {
            return 1.0;
        }
        return 0.0;
    }
}
