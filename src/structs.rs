// Structs - Used to create custom data types

// Traditional Struct
struct Color {
   red: u8,
   green: u8,
   blue: u8
}

// Tuple Struct
struct TupleColor(u8, u8, u8);

pub fn run() {
   let mut c = Color {
      red: 255,
      green: 0,
      blue: 0
   };
   c.blue = 120;

   println!("Color: RGB({}, {}, {})", c.red, c.green, c.blue);

   let mut tc = TupleColor(0, 0, 255);
   tc.1 = 80;
   println!("Color: RGB({}, {}, {})", tc.0, tc.1, tc.2);
}