// bypass errors and warnings
#![allow(dead_code)]

use std::fmt::{Debug};

// structure
#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
}

// uit structure
#[derive(Debug)]
struct Unit;

// tuple structure
#[derive(Debug)]
struct Pair(i32, f32);

// structure with two fields
#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

// a structure can reuse another structure as fields
#[derive(Debug)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
}

// enums
enum WebEvent {
  PageLoad,
  PageUnload,
  KeyPressed(char),
  Paste(String),
  Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
  match event {
    WebEvent::KeyPressed(c) => { println!("Pressed key {}", c); },
    WebEvent::PageLoad => println!("Page Loaded"),
    WebEvent::PageUnload => println!("Page Unloaded"),
    WebEvent::Click { x, y } => println!("User clicked at {} {}", x, y),
    WebEvent::Paste(s) => { println!("User pasted {}", s); },
  }
}

// type alias
#[derive(Debug)]
enum VeryVerboseEnumWithNumber {
  Add,
  Subtract,
}

type Operation = VeryVerboseEnumWithNumber;

// `Self` alias
impl VeryVerboseEnumWithNumber {
  fn run(&self, x: i32, y: i32) -> i32 {
    match self {
        Self::Add => x + y,
        Self::Subtract => x - y,
    }
  }
}


// use - use keyward can be used to avoid using the full path of the module to access name
#[derive(Debug)]
enum State {
  Beginner,
  Advance,
}

#[derive(Debug)]
enum Role {
  Student,
  Teacher,
}


// c-like enum
enum Counter {
  One,
  Two,
  Three,
}

enum Color {
  Red = 0xFF0000,
  Green = 0x00FF00,
  Blue = 0x0000FF,
}

// constatns
// const - unchangable variable
// mutable with static lifetime - but muting the variable is unsafe
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
  n > THRESHOLD
}

fn main() {
  // create structure with field init shorthand
  let name: String = String::from("Ash");
  let age: u8 = 30;
  let ash: Person = Person { age, name };
  println!("{:?}", ash);

  // instantiate a point
  let first_point: Point = Point { x: 20, y: 40 };
  let second_point: Point = Point { x : 40, y: 60 };

  println!("First point x = {}, y = {}", first_point.x, first_point.y);
  println!("Second point x = {}, y = {}", second_point.x, second_point.y);

  // make new point using struct updated syntax
  let new_point: Point = Point { x: 100, ..second_point };

  println!("The new point is {:?}", new_point);

  // destructuring with new name
  let Point { x: left_edge, y: top_edge } = new_point;
  // destructuring with same name
  let Point {x, y} = new_point;
  println!("{} {}", left_edge, top_edge);
  println!("{} {}", x, y);

  let rectangle: Rectangle = Rectangle {
    top_left: Point { x: left_edge, y: top_edge },
    bottom_right: first_point,
  };

  println!("The rectangle is = {:?}", rectangle);

  let unit: Unit = Unit;
  println!("Unit struct is = {:?}", unit);

  let pair: Pair = Pair(1, 2.11);
  println!("The pair.first = {} pair.second = {:.2}", pair.0, pair.1);

  // destructing pair structure
  let Pair(integer, decimal) = pair;
  println!("The integer value is {} and decimal is {}", integer, decimal);

  // enums
  let pressed: WebEvent = WebEvent::KeyPressed('a');
  let pasted: WebEvent = WebEvent::Paste("my_text".to_owned());
  let clicked: WebEvent = WebEvent::Click { x: 10, y: 20 };
  let load: WebEvent = WebEvent::PageLoad;
  let unload: WebEvent = WebEvent::PageUnload;

  inspect(pressed);
  inspect(pasted);
  inspect(clicked);
  inspect(load);
  inspect(unload);

  let operation: Operation = Operation::Add;
  println!("Operation is {:#?}", operation);
  let val: i32 = operation.run(1, 3);
  println!("The value after the operation is {}", val);


  use State::{ Beginner };
  use Role::*;

  let state: State = Beginner;
  let role: Role = Student;

  println!("Level of the {:#?} is {:#?}", role, state);

  println!("One is {}", Counter::One as i32);
  println!("Two is {}", Counter::Two as i64);

  println!("roses are #{:06x}", Color::Red as u32);
  println!("violets are #{:06x}", Color::Blue as u32);

  let x = 5;
  let y = 20;
  println!("{} is {} and {} is {}", x, if is_big(x) { "big" } else { "small"}, y, if is_big(y) { "big" } else { "small" });
}
