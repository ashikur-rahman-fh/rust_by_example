use std::cmp::{max, min};
use std::fmt::{self, Display, Formatter};

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


  // display
  #[derive (Debug)]
  struct MinMax(i64, i64);

  impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      return write!(f, "Min = {} Max = {}", min(self.0, self.1), max(self.0, self.1));
    }
  }

  let min_max: MinMax = MinMax(55, 10);
  println!("Debug MinMax = {:?}", min_max);
  println!("Display MinMax = {}", min_max);

  struct List(Vec<i32>);

  impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "[")?;
      let vec: &Vec<i32> = &self.0;

      for (index, value) in vec.iter().enumerate() {
        if index != 0 {
          write!(f, ", ")?;
        }
        write!(f, "{}: {}", index, value)?;
      }
      return write!(f, "]");
    }
  }

  let list: List = List(vec![5, 10, 15, 20, 25]);
  println!("My list is {}", list);


  // Formatting
  struct City {
    name: &'static str,
    latitude: f32,
    longigure: f32,
  }

  impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      let lat_c: &str = if self.latitude >= 0.0 { "N" } else { "S" };
      let lon_c: &str = if self.longigure >= 0.0 { "E" } else { "W" };

      write!(f, "{}: {:.3}°{} {:.3}°{}",
              self.name, self.latitude.abs(), lat_c, self.longigure.abs(), lon_c)
    }
  }

  for city in [
      City { name: "Dublin", latitude: 53.347778, longigure: -6.259722 },
      City { name: "Oslo", latitude: 59.95, longigure: 10.75 },
      City { name: "Vancouver", latitude: 49.25, longigure: -123.1 },
    ] {
      println!("{}", city);
    }

  struct Color {
    red: u8,
    green: u8,
    blue: u8,
  }

  impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      write!(f, "RGB ({0}, {1}, {2}) 0x{0:0>2X}{1:0>2X}{2:0>2X}", self.red, self.green, self.blue)
    }
  }
  for color in [
    Color { red: 128, green: 255, blue: 90 },
    Color { red: 0, green: 3, blue: 254 },
    Color { red: 0, green: 0, blue: 0 },
  ] {
    println!("{}", color);
  }
}
