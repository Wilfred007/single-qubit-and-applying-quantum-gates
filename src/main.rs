use rand::Rng;
use std::f64;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    fn modulus_squared(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }

    fn add(&self, other: &Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }

    fn mul(&self, other: &Complex) -> Complex {
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Qubit {
    state: [Complex; 2],
}

impl Qubit {
    fn new(alpha: Complex, beta: Complex) -> Self {
        let mut qubit = Self { state: [alpha, beta] };
        qubit.normalize();
        qubit
    }

    fn normalize(&mut self) {
        let norm = (self.state[0].modulus_squared() + self.state[1].modulus_squared()).sqrt();
        self.state[0].real /= norm;
        self.state[0].imag /= norm;
        self.state[1].real /= norm;
        self.state[1].imag /= norm;
    }

    fn measure(&mut self) -> usize {
        let probability_0 = self.state[0].modulus_squared();
        let r: f64 = rand::thread_rng().gen();
        if r < probability_0 {
            self.state = [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)];
            0
        } else {
            self.state = [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)];
            1
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct QuantumGate {
    matrix: [[Complex; 2]; 2],
}

impl QuantumGate {
    fn apply(&self, qubit: &Qubit) -> Qubit {
        let new_state = [
            self.matrix[0][0].mul(&qubit.state[0]).add(&self.matrix[0][1].mul(&qubit.state[1])),
            self.matrix[1][0].mul(&qubit.state[0]).add(&self.matrix[1][1].mul(&qubit.state[1])),
        ];
        Qubit::new(new_state[0], new_state[1])
    }
}

fn hadamard_gate() -> QuantumGate {
    let factor = 1.0 / f64::consts::SQRT_2;
    QuantumGate {
        matrix: [
            [Complex::new(factor, 0.0), Complex::new(factor, 0.0)],
            [Complex::new(factor, 0.0), Complex::new(-factor, 0.0)],
        ],
    }
}

fn main() {
    let mut qubit = Qubit::new(Complex::new(1.0, 0.0), Complex::new(0.0, 0.0));
    println!("Initial Qubit State: {:?}", qubit);

    let hadamard = hadamard_gate();
    qubit = hadamard.apply(&qubit);
    println!("After Hadamard Gate: {:?}", qubit);

    let result = qubit.measure();
    println!("Measurement Result: |{}⟩", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_operations() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let sum = a.add(&b);
        assert_eq!(sum, Complex::new(4.0, 6.0));

        let product = a.mul(&b);
        assert_eq!(product, Complex::new(-5.0, 10.0));
    }

    #[test]
    fn test_qubit_initialization() {
        let qubit = Qubit::new(Complex::new(1.0, 0.0), Complex::new(0.0, 0.0));
        assert_eq!(qubit.state[0], Complex::new(1.0, 0.0));
        assert_eq!(qubit.state[1], Complex::new(0.0, 0.0));
    }

    #[test]
    fn test_hadamard_gate() {
        let qubit = Qubit::new(Complex::new(1.0, 0.0), Complex::new(0.0, 0.0));
        let hadamard = hadamard_gate();
        let transformed = hadamard.apply(&qubit);
        let expected_factor = 1.0 / f64::consts::SQRT_2;
        assert_eq!(transformed.state[0], Complex::new(expected_factor, 0.0));
        assert_eq!(transformed.state[1], Complex::new(expected_factor, 0.0));
    }
}
