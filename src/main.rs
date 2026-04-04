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
}
