// variables are immutable by default (constant)
// Rust is block-scoped language

pub fn run() {
   let name = "John";
   let mut age = 25; // make a variable mutable
   println!("{} is {} years old", name, age);

   age = 26;
   println!("Now {} is {} years old", name, age);

   // Define constant
   const ID: i32 = 1284;
   println!("ID: {}", ID);

   // Assign multiple variables
   let ( my_name, my_age ) = ( "Shun", 17 );
   println!("My name is {}. I'm {} years old.", my_name, my_age);
}