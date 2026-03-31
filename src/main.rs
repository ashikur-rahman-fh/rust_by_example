use std::fmt::Debug;

fn main() {
  println!("Hello World!");

  // Formatted print
  // Debug

  // All types that want to use std::fmt formatting traits require an
  // implementation to be printable.
  // Automatic implementations are only provided for types such as std library.

  // The fmt::Debug trait makes it straightforward. All types can derive
  // fmt::Debug trait. But fmt::Display requires manual implementation.


  #[derive (Debug)]
  struct Point(i32, i32);
  let point: Point = Point(5, 10);
  println!("Point is {:?}, point.0 = {} point.1 = {}", point, point.0, point.1);

  // Deep structure
  #[derive (Debug)]
  struct Structure(Point);

  let deep: Structure = Structure(point);

  println!("Deep structure = {:?}", deep);
  println!("Pretty print structure = {:#?}", deep);
}
