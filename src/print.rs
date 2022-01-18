pub fn run() {
   println!("Hello from print.rs file");
   // you can not print a number
   // println!(10);

   // Formatting
   println!("{} is {} years old", "Brad", 18);

   // Positional Arguments
   println!(
      "{0} is {1} years old and {0} likes to {2}", "John", 23, "code"
   );

   // Named Arguments
   println!(
      "{name} likes to play {activity}", 
      name = "Luke",
      activity = "basketball"
   );

   // Placeholder traits
   println!("Binary: {:b} Hex: {:x} Octal: {:o}", 15, 15, 15);

   // Placeholder for debug trait
   println!("{:?}", (12, true, "hello world"));

   // Basic math
   println!("10 + 10 = {}", 10 + 10);
}