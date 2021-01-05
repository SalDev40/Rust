#![allow(non_snake_case)]
#![allow(dead_code)]
mod libs;
use crate::libs::error;
use crate::libs::generics;
use crate::libs::traits;
use crate::libs::lifetimes;

fn main() {
    // error::errMain();
    // generics::genMain();
    // traits::trMain();
    lifetimes::lifeMain();
}
