use num_bigint::BigUint;
use num_traits::{Zero, One};

pub trait Field<Rhs=Self> {

    type FT;

    fn isZero(&self) -> bool;
    fn equals(&self, rhs: Rhs) -> bool;
    fn neg(&self) -> FT;
    fn add(&self, rhs: Rhs) -> FT;
    fn subtract(&self, rhs: Rhs) -> FT;
    fn invert(&self) -> FT;
    fn mult(&self, rhs: Rhs + BigUint) -> FT;
    fn square(&self) -> FT;
    fn pow(&self, n: BigUint) -> FT;
    fn div(&self, rhs: Rhs + BigUint) -> FT;
}