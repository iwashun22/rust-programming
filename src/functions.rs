pub fn run() {
   greeting("Hello", "Jane");

   let get_sum = add(5, 5);
   println!("Sum: {}", get_sum);

   // Closure
   let n3 = 7;
   let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
   println!("Closure Sum: {}", add_nums(2, 6));
}

fn greeting(greet: &str, name: &str) {
   println!("{} {}, nice to meet you!", greet, name)
}

fn add(n1: i32, n2: i32) -> i32 {
   n1 + n2 // no semicolon
}