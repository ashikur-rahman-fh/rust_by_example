fn main() {
  // From and Into
  // From and Into are used to convert one type to another. And it's assumed
  // if A is converted to B, then it should be easy to convert B to A.

  // From - From trait allows for a type to define how to create itself from
  // another type.

  // example
  let my_str = "hello";
  let my_string = String::from(my_str);
  println!("{}", my_string);

  // We can implement our own from trait

  #[derive(Debug)]
  struct Number {
    value: i32,
  }

  impl From<i32> for Number {
    fn from(v: i32) -> Self {
      Number { value: v }
    }
  }

  let num = Number::from(44);
  println!("My number is {:#?}", num);


  // Into - Into trait is reciprocal of the From trait. It defines how to
  // convert a type into another.

  // impl Into<Number> for i32 {
  //   fn into(self) -> Number {
  //     Number{ value: self }
  //   }
  // }

  // let num: Number = 32.into();
  // println!("My number is {:#?}", num);

  // IMPORTANT NOTE: Can not implement both from and into. They are
  // interchangeable.
  // Just defining from is enough for into.

  let var: i32 = 55;
  let new_num: Number = var.into();

  println!("New number is {:#?} value is {}", new_num, new_num.value);
}
