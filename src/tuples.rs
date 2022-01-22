pub fn run() {
   let person: (&str, &str, i8) = ("Steve", "Canada", 24);
   println!("{} is from {} and is {} years old.", person.0, person.1, person.2);
}