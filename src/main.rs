fn main() {
  // if-else

  let x = 5;
  if x < 0 {
    print!("{} is negative", x);
  } else if x == 0 {
    print!("{} is zero", x);
  } else {
    print!("{} is positive", x);
  }

  let big_x = {
    if x < 10 && x > -10 {
      println!(", and is a small number, increase ten-fold");

      10 * x
    } else {
      println!(", and is a big number, halve the number");
      x / 2
    }
  };

  println!("{} -> {}", x, big_x);
}
