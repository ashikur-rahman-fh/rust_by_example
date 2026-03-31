use std::any::type_name_of_val;

fn main() {
  // signed integers: i8, i16, i32, i64, i128, isize
  // unsigned integers: u8, u16, u32, u64, u128, usize

  let var: i8 = 10;
  println!("Value of var is {} and it's type is {}", var, type_name_of_val(&var));

  let var: u16 = 1 << 15;
  println!("Value of var is {} and it's type is {}", var, type_name_of_val(&var));

  // floating point: f32, f64
  let var: f64 = 1f64 / 7 as f64;
  println!("Value of var is {} and it's type is {}", var, type_name_of_val(&var));

  // char: a 4 bytes scaler value
  let var: char = 'a';
  println!("Value of var is {} and it's type is {}", var, type_name_of_val(&var));

  // bool: true or false
  let var: bool = true;
  println!("Value of var is {} and it's type is {}", var, type_name_of_val(&var));

  // unit type: () -> possibly an empty tuple
  let var: (i32, i32) = (1, 2);
  println!("Value of var is {:?} and it's type is {}", var, type_name_of_val(&var));


  // Integers can be represented with binary, octal and hexadecimal notation
  let regular: i32 = 31;
  let binary: i32 = 0b11111;
  let octal: i32 = 0o37;
  let hex: i32 = 0x1F;

  println!("The values are same {} = {} = {} = {}", regular, binary, octal, hex);

  let x: i32 = 5;
  let y: i32 = 10;

  // arithmetic operations
  println!("{} + {} = {}",x, y, x + y);
  println!("{} - {} = {}", x, y, x - y);
  println!("{} x {} = {}", x, y, x * y);
  println!("{} / {} = {}", x, y, x as f32 / y as f32);

  // bitwise operations
  println!("{} AND {} = {}", x, y, x & y);
  println!("{} OR {} = {}", x, y, x | y);
  println!("{} XOR {} = {}", x, y, x ^ y);
  println!("{} << {} = {}", x, y, x << y);
  println!("{} >> {} = {}", x, y, x >> y);

  // underscore to improve readability
  println!("One million = {}", 1_000_000);
}
