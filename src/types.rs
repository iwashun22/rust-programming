/*
--- Primitive Types ---
   INTEGER: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)

   FLOATS: f32, f64

   BOOLEAN (bool)

   CHARACTERS (char) char is not a string!
*/

/*
   Difference between i(int) and u(unsigned)
      i - can be any integer number include negative value.
      u - can not be negative value
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

   // Boolean
   let is_active: bool = true;

   // Get boolean from expression
   let is_greater = 7 > 9;

   // Char ** needs to be a single quotes **
   let a1 = 'a';

   // Specify the unicode
   let emoji_face = '\u{1F606}';

   println!("{:?}", (_x, _y, is_active, is_greater, a1, emoji_face));
}