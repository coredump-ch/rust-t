enum Colors {
  Red,
  Green,
  Blue,
}
use Colors::*;

fn main() {
  draw(Red);

  draw(Blue);
}

fn draw(color: Colors) {
  match color {
    Red => 0xff0000,
    // Green => 0x00ff00,
    Blue => 0x0000ff,
  }; // no return
}
