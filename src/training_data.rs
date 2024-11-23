use rand::Rng;

pub struct TrainingData {
    pub inputs: Vec<Vec<f64>>,
    pub targets: Vec<Vec<f64>>,
}

impl TrainingData {
    pub fn new(inputs: &Vec<Vec<f64>>, targets: &Vec<Vec<f64>>) -> Self {
        if inputs.len() != targets.len() {
            panic!("Inputs and target sizes do not match");
        }
        TrainingData {
            inputs: inputs.clone(),
            targets: targets.clone(),
        }
    }

    pub fn shuffle(&self) -> TrainingData {
        let mut shuf_inputs = self.inputs.clone();
        let mut shuf_targets = self.targets.clone();
        let mut rng = rand::thread_rng();

        for i in 0..self.inputs.len() {
            let rng_idx = rng.gen_range(i..self.inputs.len());

            let tmp = shuf_inputs[i].clone();
            shuf_inputs[i] = shuf_inputs[rng_idx].clone();
            shuf_inputs[rng_idx] = tmp.clone();

            let tmp = shuf_targets[i].clone();
            shuf_targets[i] = shuf_targets[rng_idx].clone();
            shuf_targets[rng_idx] = tmp.clone();
        }

        TrainingData {
            inputs: shuf_inputs,
            targets: shuf_targets,
        }
    }
}

#[test]
fn shuffle() {
    let inputs = vec![
        vec![1.0, 1.0],
        vec![1.0, 0.0],
        vec![0.0, 1.0],
        vec![0.0, 0.0],
    ];
    let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];

    let training_data = TrainingData::new(&inputs, &targets);
    let shuffled = training_data.shuffle();
    assert_ne!(shuffled.inputs, training_data.inputs);
    assert_ne!(shuffled.targets, training_data.targets);
    assert!(shuffled.inputs.contains(&training_data.inputs[0]));
    assert!(shuffled.inputs.contains(&training_data.inputs[1]));
    assert!(shuffled.inputs.contains(&training_data.inputs[2]));
    assert!(shuffled.inputs.contains(&training_data.inputs[3]));
    assert!(shuffled.targets.contains(&training_data.targets[0]));
    assert!(shuffled.targets.contains(&training_data.targets[1]));
    assert!(shuffled.targets.contains(&training_data.targets[2]));
    assert!(shuffled.targets.contains(&training_data.targets[3]));
}
