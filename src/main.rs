use core::fmt;
use std::{fmt::Error, num::ParseIntError, str::FromStr};

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

  // TryFrom and TryInto
  // Similar to try and into but used for fallible conversions.

  #[derive(Debug, PartialEq)]
  struct EvenNumber(i32);

  impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
      if value % 2 == 0 {
        Ok(EvenNumber(value))
      } else {
        Err(())
      }
    }
  }

  let good_even: EvenNumber = EvenNumber::try_from(100).unwrap_or(EvenNumber(0));
  let bad_even: EvenNumber = EvenNumber::try_from(101).unwrap_or(EvenNumber(0));

  println!("The good even is {:#?} and the bad even is {:#?}", good_even, bad_even);

  assert_eq!(EvenNumber::try_from(2), Ok(EvenNumber(2)));
  assert_eq!(EvenNumber::try_from(3), Err(()));
  assert_ne!(EvenNumber::try_from(5), Ok(EvenNumber(5)));

  assert_eq!(4i32.try_into(), Ok(EvenNumber(4)));
  assert_ne!(5i32.try_into(), Ok(EvenNumber(5)));

  // To and From Strings
  // converting to string
  // similar to to_string method but here just to implement ToString trait.

  struct Circle {
    radius: i32,
  }

  // impl ToString for Circle {
  //   fn to_string(&self) -> String {
  //     let val = format!("ToString Circle with radius {}", self.radius);
  //     return val;
  //   }
  // }

  // Rather doing that directly, we should try implementing the Display trait,
  // which automatically provides ToString and also allows printing type
  impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Circle of radius {}", self.radius)
    }
  }

  let circle = Circle { radius: 6 };
  println!("{}", circle.to_string());

  // parsing a string
  // it is always useful to convert a string to many types. One of the common
  // operations is to convert string to number.

  let parse: i32 = "5".parse().unwrap();
  let turbo_parsed = "55".parse::<i32>().unwrap();

  let sum = parse + turbo_parsed;
  println!("The sum is {}", sum);

  // for user defined type
  // we need to implement the FromStr trait

  impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s.trim().parse() {
        Ok(num) => Ok(Circle { radius: num }),
        Err(e) => Err(e),
      }
    }
  }

  let rad = "    3   ";
  let circle = rad.parse::<Circle>().unwrap();
  println!("{}", circle.to_string());
}
