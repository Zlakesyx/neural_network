use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn random(rows: usize, cols: usize) -> Self {
        let mut buffer: Vec<f64> = Vec::with_capacity(rows * cols);

        for _ in 0..buffer.capacity() {
            let num = rand::thread_rng().gen_range(0.0..1.0);
            buffer.push(num);
        }

        Matrix {
            rows,
            cols,
            data: buffer,
        }
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        let mut buffer: Vec<f64> = Vec::with_capacity(rows * cols);

        for _ in 0..buffer.capacity() {
            buffer.push(0.0);
        }

        Matrix {
            rows,
            cols,
            data: buffer,
        }
    }

    pub fn from_vec(data: &Vec<f64>, rows: usize, cols: usize) -> Self {
        if data.len() != rows * cols {
            println!("{:?}, {}, {}", data, data.capacity(), rows*cols);
            panic!("Vector capacity does not match rows * cols");
        }

        Matrix {
            rows,
            cols,
            data: data.to_vec(),
        }
    }

    pub fn from_vec_2d(data: Vec<Vec<f64>>) -> Self {
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data: data.into_iter().flatten().collect::<Vec<f64>>(),
        }
    }

    pub fn map(&self, function: &dyn Fn(f64) -> f64) -> Self {
        Matrix::from_vec(
            &(self.data)
                .clone()
                .into_iter()
                .map(|x| function(x))
                .collect(),
            self.rows,
            self.cols,
        )
    }

    pub fn add(&self, other: &Matrix) -> Self {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Rows or columns do not match between the matrices");
        }

        let mut buffer: Vec<f64> = Vec::with_capacity(self.rows * self.cols);

        for i in 0..self.data.len() {
            let num = self.data[i] + other.data[i];
            buffer.push(num);
        }

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: buffer,
        }
    }

    pub fn subtract(&self, other: &Matrix) -> Self {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Rows or columns do not match between the matrices");
        }

        let mut buffer: Vec<f64> = Vec::with_capacity(self.rows * self.cols);

        for i in 0..self.data.len() {
            let num = self.data[i] - other.data[i];
            buffer.push(num);
        }

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: buffer,
        }
    }

    pub fn dot_multiply(&self, other: &Matrix) -> Self {
        if self.cols != other.rows {
            panic!("Size mismatch when dot multiplying matrices. Matrix A columns must match Matrix B rows.");
        }

        let mut buffer: Vec<f64> = vec![0.0; self.rows * other.cols];

        for a_r in 0..self.rows {
            for b_c in 0..other.cols {
                let mut sum: f64 = 0.0;

                for a_c in 0..self.cols {
                    sum += self.data[a_r * self.cols + a_c] * other.data[a_c * other.cols + b_c];
                }

                buffer[a_r * other.cols + b_c] = sum;
            }
        }

        Matrix {
            rows: self.rows,
            cols: other.cols,
            data: buffer,
        }
    }

    pub fn multiply(&self, other: &Matrix) -> Self {
        if self.cols != other.cols || self.rows != other.rows {
            panic!("Size mismatch when multiplying matrices");
        }

        let mut buffer: Vec<f64> = Vec::with_capacity(self.rows * self.cols);

        for row in 0..self.rows {
            for col in 0..self.cols {
                let idx = row * self.cols + col;
                buffer.push(self.data[idx] * other.data[idx]);
            }
        }

        Matrix {
            rows: self.rows,
            cols: other.cols,
            data: buffer,
        }
    }

    pub fn transpose(&self) -> Self {
        let mut buffer = Vec::with_capacity(self.rows * self.cols);
        for row in 0..self.rows {
            for col in 0..self.cols {
                buffer.push(self.data[col * self.rows + row]);
            }
        }
        Matrix {
            rows: self.cols,
            cols: self.rows,
            data: buffer,
        }
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let spacer = "--------------------";
        write!(f, "\nrows: {}, cols: {}", self.rows, self.cols);
        write!(f, "\n{}\n", spacer);
        for row in 0..self.rows {
            write!(f, "|");
            for col in 0..self.cols {
                write!(f, " {} |", self.data[row * self.cols + col]);
            }
            write!(f, "\n");
        }
        write!(f, "{}\n", spacer)
    }
}

#[test]
#[should_panic]
fn add() {
    let matrix = Matrix::random(2, 2);
    let other = Matrix::random(2, 3);
    matrix.add(&other);
}

#[test]
#[should_panic]
fn subtract() {
    let matrix = Matrix::random(3, 2);
    let other = Matrix::random(2, 2);
    matrix.subtract(&other);
}

#[test]
fn dot_multiply() {
    let mut matrix = Matrix::random(2, 3);
    let mut other = Matrix::random(3, 2);

    let a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    matrix.data = a;

    let b: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    other.data = b;

    let expected_result = vec![22.0, 28.0, 49.0, 64.0];

    let result = matrix.dot_multiply(&other);

    assert_eq!(result.rows, matrix.rows);
    assert_eq!(result.cols, other.cols);
    assert_eq!(result.data, expected_result);
}

#[test]
fn feed_forward() {
    let mut input = Matrix::random(1, 2);
    input.data = vec![1.0, 3.0];

    let mut hidden_1 = Matrix::random(2, 2);
    hidden_1.data = vec![1.0, 1.0, 1.0, 1.0];

    let mut hidden_2 = Matrix::random(2, 2);
    hidden_2.data = vec![2.0, 2.0, 2.0, 2.0];

    let mut output = Matrix::random(2, 1);
    output.data = vec![3.0, 3.0];
    let bias = 1.0;

    hidden_1 = input.dot_multiply(&hidden_1);
    assert_eq!(hidden_1.data, [4.0, 4.0]);
    hidden_1.data = hidden_1.data.iter().map(|x| x + bias).collect::<Vec<f64>>();
    assert_eq!(hidden_1.data, [5.0, 5.0]);

    hidden_2 = hidden_1.dot_multiply(&hidden_2);
    assert_eq!(hidden_2.data, [20.0, 20.0]);
    hidden_2.data = hidden_2.data.iter().map(|x| x + bias).collect::<Vec<f64>>();
    assert_eq!(hidden_2.data, [21.0, 21.0]);

    output = hidden_2.dot_multiply(&output);
    assert_eq!(output.data, [126.0]);
    output.data = output.data.iter().map(|x| x + bias).collect::<Vec<f64>>();
    assert_eq!(output.data, [127.0]);
}

#[test]
fn transpose() {
    let mut a = Matrix::random(1, 2);
    a.data = vec![1.0, 2.0];

    println!("{}", &a);

    let mut expected = Matrix::random(2, 1);
    expected.data = vec![1.0, 2.0];

    let transposed = a.transpose();

    println!("{}", &transposed);

    assert_eq!(transposed, expected);
}
