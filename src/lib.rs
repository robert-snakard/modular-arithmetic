extern crate typenum;
#[allow(dead_code)]

use std::fmt;
use std::ops::{Add};
use std::marker::PhantomData;
use typenum::*;

#[derive(Debug)]
#[derive(Clone, Copy)]
struct ModInt<M> (i32, PhantomData<M>);
impl<M> ModInt<M> {
    fn new(n: i32) -> ModInt<M> {
        ModInt(n, PhantomData)
    }
}

impl<M: Integer> Add for ModInt<M> {
    type Output = ModInt<M>;

    fn add(self, _rhs: ModInt<M>) -> ModInt<M> {
        let sum = self.0 + _rhs.0;
        ModInt( sum % <M as Integer>::to_i32(), PhantomData )
    }
}

impl<M> fmt::Display for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}




#[test]
fn it_works() {
    let a = ModInt::<P5>::new(3);
    let b = ModInt::<P5>::new(4);
    let sum = a+b;
    println!("{} + {} = {}", a, b, sum);
}

