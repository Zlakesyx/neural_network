use crate::{
    activation::{Activation, SIGMOID},
    matrix::Matrix,
    training_data::TrainingData,
};

pub struct Network {
    layer_sizes: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    layer_outputs: Vec<Matrix>,
    activation: Activation,
    learning_rate: f64,
}

impl Network {
    pub fn new(layer_sizes: Vec<usize>, activation: Activation, learning_rate: f64) -> Network {
        let mut weights: Vec<Matrix> = vec![];
        let mut biases: Vec<Matrix> = vec![];

        for i in 0..layer_sizes.len() - 1 {
            weights.push(Matrix::random(layer_sizes[i + 1], layer_sizes[i]));
            biases.push(Matrix::random(layer_sizes[i + 1], 1));
        }

        Network {
            layer_sizes,
            weights,
            biases,
            layer_outputs: vec![],
            activation,
            learning_rate,
        }
    }

    pub fn feed_forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        if inputs.len() != self.layer_sizes[0] {
            panic!("Invalid number of inputs");
        }

        let mut output: Matrix = Matrix::from_vec(&inputs, inputs.len(), 1);
        self.layer_outputs = vec![output.clone()];

        for layer in 0..self.layer_sizes.len() - 1 {
            output = self.weights[layer]
                .dot_multiply(&output)
                .add(&self.biases[layer])
                .map(self.activation.function);

            self.layer_outputs.push(output.clone());
        }

        output.transpose().data
    }

    pub fn back_propagation(&mut self, outputs: Vec<f64>, targets: Vec<f64>) {
        if targets.len() != self.layer_sizes[self.layer_sizes.len() - 1] {
            panic!("Number of targets does not equal the number of output layer nodes");
        }

        let output_matrix = Matrix::from_vec(&outputs, 1, outputs.len()).transpose();
        let mut errors = Matrix::from_vec(&targets, 1, targets.len()).transpose()
            .subtract(&output_matrix);
        let mut gradients = output_matrix.map(self.activation.derivative);

        for layer in (0..self.layer_sizes.len() - 1).rev() {
            gradients = gradients.multiply(&errors).map(&|x| x * self.learning_rate);

            self.weights[layer] = self.weights[layer]
                .add(&gradients.dot_multiply(&self.layer_outputs[layer].transpose()));
            self.biases[layer] = self.biases[layer].add(&gradients);

            errors = self.weights[layer].transpose().dot_multiply(&errors);
            gradients = self.layer_outputs[layer].map(self.activation.derivative);
        }
    }

    pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, ephochs: u16) {
        let mut data = TrainingData::new(&inputs, &targets);
        for epoch in 0..=ephochs {
            if epoch % 100 == 0 {
                println!("Ephoch: {}", epoch);
            }
            for i in 0..data.inputs.len() {
                let outputs = self.feed_forward(data.inputs[i].clone());
                self.back_propagation(outputs, data.targets[i].clone());
            }
            data = data.shuffle();
        }
    }
}

#[test]
fn xor() {
    let inputs = vec![
        vec![1.0, 1.0],
        vec![1.0, 0.0],
        vec![0.0, 1.0],
        vec![0.0, 0.0],
    ];
    let targets = vec![
        vec![0.0],
        vec![1.0],
        vec![1.0],
        vec![0.0],
    ];
    let mut network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);

    // Test pretrained
    println!("Pre-Trained");
    for i in 0..inputs.len() {
        let pre_trained = network.feed_forward(inputs[i].clone());
        println!("input: {:#?}, result: {:#?}", inputs[i], pre_trained);
    }

    // Test trained
    network.train(inputs.clone(), targets, 10000);
    println!("Post-Trained");
    println!("input: {:#?}, result: {:#?}", inputs[0], network.feed_forward(inputs[0].clone()));
    println!("input: {:#?}, result: {:#?}", inputs[1], network.feed_forward(inputs[1].clone()));
    println!("input: {:#?}, result: {:#?}", inputs[2], network.feed_forward(inputs[2].clone()));
    println!("input: {:#?}, result: {:#?}", inputs[3], network.feed_forward(inputs[3].clone()));
}
