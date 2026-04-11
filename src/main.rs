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
}
