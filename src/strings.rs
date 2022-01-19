// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
   // let hello = "hello" // this is immutable
   let mut hello = String::from("Hello ");

   // Get length
   println!("Length: {}", hello.len());

   // Push is for Char
   hello.push('W');

   hello.push_str("orld!");

   println!("{}", hello)
}