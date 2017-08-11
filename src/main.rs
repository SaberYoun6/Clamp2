 // just so i can use clamp in rust 
 // someone started this project and didn't put any work into to it 
 // this will also be the first cargo that i will actaul upload to the cargo page 
 // I need clamp to continue 
extern crate compare;
use compare::{Compare };
use std::cmp::Ordering;
use std::cmp::PartialOrd;
struct Clamp<T>(T);
fn clamp( v :T, lo : T, hi : T) -> T {
    let cmp = T();  
    return  assert!(hi.compares_ne(hi,lo),"{:?}= hi, {:?}=low",hi,lo),
    v.compares_lt(v,lo) ? lo : hi.compares_gt(hi,v) ? hi : v;
}
