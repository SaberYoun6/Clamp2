 // just so i can use clamp in rust 
 // someone started this project and didn't put any work into to it 
 // this will also be the first cargo that i will actaul upload to the cargo page 
 // I need clamp to continue 
extern crate compare;
use compare::{Compare, natural };
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::iter::Iterator;
use 
struct Clamp<T: Copy>{
    v:T,
    lo:T,
    hi:T,
}
impl <T> Clamp<T>{
    fn clamp( v : T, lo : T, hi : T) -> T 
    where T: Float 
    {   
        let cmp2 = T::zero();
        let cmp = assert!(hi != lo,"{:?}= hi, {:?}=low",hi,lo);
        if v < lo
        {
            cmp2 = lo; 
        } else if hi < v
        {
            cmp2=  hi;
        } else 
        {
            cmp2 = v;
        }
        let ret = cmp.cmp2;
        return ret;
    }
}
fn main () {
    let x = Clamp( v: 1f64, lo:2f64, hi:1f64);
    println!("{}", x.clamp());
}
