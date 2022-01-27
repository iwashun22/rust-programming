// Structs - Used to create custom data types

// Traditional Struct
struct Color {
   red: u8,
   green: u8,
   blue: u8
}

// Tuple Struct
struct TupleColor(u8, u8, u8);

pub fn run() {
   let mut c = Color {
      red: 255,
      green: 0,
      blue: 0
   };
   c.blue = 120;

   println!("Color: RGB({}, {}, {})", c.red, c.green, c.blue);

   let mut tc = TupleColor(0, 0, 255);
   tc.1 = 80;
   println!("Color: RGB({}, {}, {})", tc.0, tc.1, tc.2);

   handle_person();
}

struct Person {
   first_name: String, // You can't do &str
   last_name: String,
   age: u8
}

impl Person {
   // Construct Person
   fn new(first: &str, last: &str, age: u8) -> Person {
      Person {
         first_name: first.to_string(),
         last_name: last.to_string(),
         age: age
      }
   }
   
   // Get full name
   fn full_name(&self) -> String {
      format!("{} {}", self.first_name, self.last_name)
   }

   // Set last name
   fn set_last_name(&mut self, last: &str) {
      self.last_name = last.to_string();
   }

   // Get person to tuple
   fn to_tuple(&self) -> (String, String, u8) {
      (self.first_name.to_string(), self.last_name.to_string(), self.age)
   }
}

fn handle_person() {
   let mut p = Person::new("James", "Smith", 23);
   // println!("Person: {} {}", p.first_name, p.last_name);
   println!("Person: {}", p.full_name());
   println!("Age: {}", p.age);

   p.set_last_name("Williams");
   println!("Person: {}", p.full_name());

   println!("Person Tuple: {:?}", p.to_tuple());
}