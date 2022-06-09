use num_bigint::BigUint;
use num_traits::{Zero, One};

/* Generic finite field trait. Arithemetic operations
   over finite fields are implemented. Each type for
   which field is implemented is a field element type,
   representing a particular element of that field. */

pub trait Field<Rhs=Self> {

    // Identity check
    fn isZero(&self) -> bool;

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
    fn mult(&self, rhs: Rhs + BigUint) -> Self;

    // Squaring
    fn square(&self) -> Self;

    // Power
    fn pow(&self, n: BigUint) -> Self;

    // Division
    fn div(&self, rhs: Rhs + BigUint) -> Self;
}

// Euclidean GCD algorithm to invert num under field
fn invert(num: BigUint, modulus: BigUint) -> BigUint {
    const _0:BigUint = BigUint(0);
    const _1:BigUint = BigUint(1);

    // Check if operation is valid
    if (num == _0) || (modulus <= _0) {
        
    } 
}