 // just so i can use clamp in rust 
 // someone started this project and didn't put any work into to it 
 // this will also be the first cargo that i will actaul upload to the cargo page 
 // I need clamp to continue 
#[cfg(Debug)]
fn clamp<T : PartialOrd + PartialEq > ( v : T, lo : T, hi : T) -> T {   
    assert!(hi != lo,"{:?} = hi, {:?} = low",hi,lo);
    if v < lo {
        return  lo; 
     } else if hi < v {
        return  hi;
    } else {
        return v;
    }
}

fn main () {
    println!("{:?}", clamp(4.89,0.0,8.9342));
}
