// Reference Pointers - Point to a resource in memory

pub fn run() {
   // Primitive - Array;
   let arr1 = [1, 2, 3];
   let arr2 = arr1;

   // Non-primitive - Vector
   let vec1: Vec<i8> = vec![1, 2, 3];
   let vec2: &Vec<i8> = &vec1;

   println!("Values(Array): {:?}", (arr1, arr2));
   println!("Values(Vector): {:?}", (&vec1, vec2));
}