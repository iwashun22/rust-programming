enum Movement {
   UP,
   DOWN,
   LEFT,
   RIGHT
}

fn move_avatar(m: Movement) {
   // match is similar to switch
   match m {
      Movement::UP => println!("Avatar moving up"),
      Movement::DOWN => println!("Avatar moving down"),
      Movement::LEFT => println!("Avatar moving left"),
      Movement::RIGHT => println!("Avatar moving right")
   }
}

pub fn run() {
   let avatar1 = Movement::LEFT;
   let avatar2 = Movement::UP;
   let avatar3 = Movement::RIGHT;
   let avatar4 = Movement::DOWN;

   move_avatar(avatar1);
   move_avatar(avatar2);
   move_avatar(avatar3);
   move_avatar(avatar4);
}