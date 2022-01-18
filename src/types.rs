/*
--- Primitive Types ---
   INTEGER: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)

   FLOATS: f32, f64

   BOOLEAN (bool)

   CHARACTERS (char) char is not a string!
*/

pub fn run() {
   // Default is "i32"
   let _x = 1; 

   // Default is "f64"
   let _y = 3.14;

   // Add expplicit type
   let _z: i64 = 9730162052;

   // Find max size
   println!("Max i32: {}", std::i32::MAX);
   println!("Max i64: {}", std::i64::MAX);

   let is_active: bool = true;
   println!("{:?}", (_x, _y, _z, is_active));
}