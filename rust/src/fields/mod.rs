use num_bigint::BigUint;
use num_traits::{Zero, One};

mod curve_parameters;

/* Generic finite field trait. Arithemetic operations
   over finite fields are implemented. Each type for
   which field is implemented is a field element type,
   representing a particular element of that field. */

pub trait Field<Rhs=Self> {

    // Identity check
    fn is_zero(&self) -> bool;

    // Equality check
    fn equals(&self, rhs: Rhs) -> bool;

    // Negation
    fn neg(&self) -> Self;

    // Addition
    fn add(&self, rhs: Rhs) -> Self;

    // Subtraction
    fn subtract(&self, rhs: Rhs) -> Self;

    // Inversion
    fn invert(&self) -> Self;

    // Multiplication
    fn mult(&self, rhs: Rhs) -> Self;

    // Squaring
    fn square(&self) -> Self;

    // Power
    fn pow(&self, n: BigUint) -> Self;

    // Division
    fn div(&self, rhs: Rhs) -> Self;
}

// Euclidean inversion to invert num under field
pub fn invert(num: BigUint, modulo: BigUint) -> BigUint {

    // Check if operation is valid
    if (num == Zero::zero()) || (modulo <= One::one()) {
        panic!("Invalid input!");
    }

    // Extended GCD algorithm
    let mut x: BigUint = Zero::zero();
    let mut y: BigUint = One::one();
    let mut u: BigUint = Zero::zero();
    let mut v: BigUint = One::one();

    let mut a: BigUint = num.clone();
    let mut b: BigUint = modulo.clone();

    while a != Zero::zero() {
        let q = &a / &b;
        let r = &a % &b;

        let m = &x-&u*&q;
        let n = &y-&v*&q;

        a = b;
        b = r;
        x = u;
        y = v;
        u = m;
        v = n;
    }

    let gcd: BigUint = b;

    /* The previous algorithm returns x and y such that
       num*x + y*mod = gcd(x,y). */

    if gcd != One::one() {
        panic!("Inverse does not exist!");
    }

    return x % &modulo;
}

pub struct PF {
    pub ORDER: BigUint,
    pub value: BigUint    
}

impl Field for PF {
    
}