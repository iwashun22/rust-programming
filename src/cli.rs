use std::env;

pub fn run() {
   let args: Vec<String> = env::args().collect();
   println!("Args: {:?}", args);

   let cmd: String;
   let status = "100%";

   if type_of(&args[1]) == "alloc::string::String" {
      // println!("{}", type_of(&args[1]));
      cmd = args[1].clone().to_string();
      println!("Command: {}", cmd);

      if cmd == "hello" {
         println!("Hi, how are you?");
      }
      else if cmd == "status" {
         println!("Status is {}", status);
      }
      else {
         println!("\"{}\" is not a valid command", cmd);
      }
   }
}

fn type_of<T>(_: &T) -> &str {
   std::any::type_name::<T>()
}