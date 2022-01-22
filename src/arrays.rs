// Arras - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
   // typed this array to an arrays of integer that contains 5 elements
   let mut numbers: [i16; 5] = [1, 0, 3, 4, 5];

   // Re-assign value
   numbers[1] = 2;

   println!("{:?}", numbers);

   // Get single value
   println!("Single Value: {}", numbers[0]);

   // Get array length
   println!("Array Length: {}", numbers.len());

   // Arrays are stack allocated
   println!("Array occupies {} bytes", mem::size_of_val(&numbers));

   // Get Slice
   let slice: &[i16] = &numbers[0..2];
   println!("Slice: {:?}", slice);
}