mod perceptron;
use perceptron::perceptron::perceptron::Perceptron;
use perceptron::sample::sample::Sample;

pub struct Letter {
    bits_input: [i32; 15],
    letter: String,
}

fn main() {
    let letters = [
        Letter {
            bits_input: [0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1],
            letter: "A".to_string(),
        },
        Letter {
            letter: "E".to_string(),
            bits_input: [1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1],
        },
        Letter {
            letter: "I".to_string(),
            bits_input: [0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0],
        },
        Letter {
            letter: "O".to_string(),
            bits_input: [1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1],
        },
        Letter {
            letter: "U".to_string(),
            bits_input: [1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1],
        },
    ];

    let mut perceptron_a = Perceptron::new(15, 5, 0.5);

    perceptron_a.set_weights(&[
        1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0, 2.1, 2.2, 2.3, 2.4, 2.5,
    ]);

    for letter in letters.iter() {
        let sample = Sample::new_sample(
            letter.bits_input.iter().map(|&x| x as f32).collect(),
            if letter.letter.eq("A") { 1.0 } else { 0.0 },
        );
        perceptron_a.push_sample(sample);
    }

    perceptron_a.train();

    println!(
        "The accuracy of A is {}%",
        perceptron_a.get_accuracy() * 100.0
    );
}
