// Vectors - Resizable arrays

use std::mem;

pub fn run() {
   let mut numbers: Vec<i32> = vec![1, 2, 0, 4];

   // Re-assign value
   numbers[2] = 3;

   // Add on to vector
   numbers.push(5);
   numbers.push(6);

   // Pop off last value
   numbers.pop();

   println!("{:?}", numbers);
   println!("Vector Length: {}", numbers.len());
   println!("Vector occpies {} bytes", mem::size_of_val(&numbers));

   // Get Slice
   let slice: &[i32] = &numbers[0..3];
   println!("Slice: {:?}", slice);

   // Loop through vector values
   for x in &numbers {
      println!("Number: {}", x);
   }

   // Loop and mutate values
   for x in numbers.iter_mut() {
      *x *= 2;
   }
   println!("Iterated and mutated numbers: {:?}", numbers);
}