pub fn run() {
   let age: u8 = 18;
   let have_license: bool = true;

   if age >= 18 && have_license {
      println!("EMPLOYER: What role you looking for?");
   } else if age < 18 {
      println!("EMPLOYER: Sorry, you are under the age.");
   } else {
      println!("EMPLOYER: Sorry, you need to get the license.");
   }
}