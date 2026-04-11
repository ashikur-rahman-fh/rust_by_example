// Suppress all error s from casts which overflow
// #![allow(overflowing_literals)]

use core::f32;
use std::any::type_name_of_val;

fn main() {
  // casting
  /*
    Rust does not provide implicit type conversion between primitive types.
    For explicit casting we can use `as` keyword.

    It mostly follow C conventions except in case where C has overflow behavior.
   */

  let decimal: f64 = 97.123;
  println!("Value of decimal is = {} and type is {}", decimal, type_name_of_val(&decimal));

  // explicit conversion
  let integer: u8 = decimal as u8;
  println!("value of integer is {} and type is {}", integer, type_name_of_val(&integer));

  // only u8 can be converted to char
  let char: char = integer as char;
  println!("Casting: {} -> {} -> {}", decimal, integer, char);

  let sci : f32 = 1e5;
  println!("Value of sci is {} and type is {}", sci, type_name_of_val(&sci));


  let nan = f32::NAN;
  println!("Value of nan is {} and type is {}", nan, type_name_of_val(&nan));


  // some runtime improvement with unsafe operations
  unsafe {
    let not_safe = 300.00_f32.to_int_unchecked::<u8>();
    println!("Value of not safe is casted to {}", not_safe);
  }

  // literals
  // Numeric literal can be type annotated by adding suffix;
  let x = 1u8;
  let y = 2i32;
  let z = 3f64;

  // un-suffixed are types depending on how they are used
  let i = 1;
  let f = 1.0;

  println!("Value of x is {} and size is {}", x, std::mem::size_of_val(&x));
  println!("Value of y is {} and size is {}", y, std::mem::size_of_val(&y));
  println!("Value of z is {} and size is {}", z, std::mem::size_of_val(&z));
  println!("Value of i is {} and size is {}", i, std::mem::size_of_val(&i));
  println!("Value of f is {} and size is {}", f, std::mem::size_of_val(&f));

  // inference
  /*
    The type inference engine is pretty smart. It does more than looking at the
    type of the value expressing during an initialization. If also looks at how
    variable is used afterwards to infer type.
   */
  let elem: u8 = 10;

  let mut vec = Vec::new();
  vec.push(elem);

  println!("vec = {:#?} type is {}", vec, type_name_of_val(&vec));
}
