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
   let  minx = 0.0;
   let  x=3.4135;
   let  y=11.423456;
   let  z=31.3451235;
    assert_eq!(clamp(x,minx,y),x);
    assert_eq!(clamp(x,minx,x),false);
    assert_eq!(clamp(z,minx,y),y);
    assert_eq!(clamp(y,minx,x),x);
    assert_eq!(clamp(4.89,0.0,8.9342),4.89);
    assert_eq!(clamp(9.99,minx,912.1234),9.99);
    println!("{:?}", clamp(4.89,0.0,8.9342));
}
