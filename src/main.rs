 // just so i can use clamp in rust 
 // someone started this project and didn't put any work into to it 
 // this will also be the first cargo that i will actaul upload to the cargo page 
 // I need clamp to continue 
extern crate compare;
use compare::{Compare, Natural };
use std::cmp::Ordering;
use std::cmp::PartialOrd;
struct Clamp<T>{
    v:T,
    lo:T,
    hi:T,
}
impl <T> Clamp<T>{
    fn clamp( v :T, lo : T, hi : T) -> T {   
        let cmp2 = v;
        let comp = natural();
        let cmp = assert!(comp.compares_ne(hi,lo),"{:?}= hi, {:?}=low",hi,lo);
        if comp.compares_lt(v,lo){
            cmp2 = lo; 
        } else if comp.compares_lt(hi,v){
            cmp2=  hi;
        } else {
            cmp2 = v;
        }
        let ret = cmp.cmp2;
        return ret;
    }
}
fn main () {
    clamp(0.01,0.0,25.02);
}
