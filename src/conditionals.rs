pub fn run() {
   let age: u8 = 21;
   let have_license: bool = false;

   if age >= 18 && have_license {
      println!("EMPLOYER: What role you looking for?");
   } else if age < 18 {
      println!("EMPLOYER: Sorry, you are under the age.");
   } else {
      println!("EMPLOYER: Sorry, you need to get the license.");
   }
}