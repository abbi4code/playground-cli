// Todo: Create a number-guessing game where the program generates a random number, and the user tries to guess it. Use std::io for input and rand crate for random number generation.
use std::io;
use rand::Rng;
use std::cmp::Ordering;
//this ordering is the result of the comparision between two vals

// ! build a tic to toe game and ask user to play one give him option use modules devide it use err handling and more 

// Todo: track no of attempts

// take it no of player then then cal how many each of em take to guess the correct ans then display in the final leaderboard
//Stylish Output
// todo: Use ASCII art or colorful text to make the game look more appealing. You can use the colored crate:

enum Difficulty {
    Easy,
    Medium,
    Hard,
}
impl Difficulty {
    fn from_input(input: &str) -> Option<Self> {
        match input.trim().to_lowercase().as_str() {
            "easy" => Some(Difficulty::Easy),
            "medium" => Some(Difficulty::Medium),
            "hard" => Some(Difficulty::Hard), 
            _ => None,
        }
    }

    fn desc(&self) -> &str {
        match self {
            Difficulty::Easy => "Esy lv: kya hua fat gayi kya",
            Difficulty::Medium => "Med lv: yeh hue na baaat",
            Difficulty::Hard => "Hard lv: ab hoga mauth ka n naach",
        }
    }
}

fn main() {
    println!("Welcome to the guessing game");

    println!("Enter your difficulty -Easy -Medium -Hard");
    let mut difficulty = String::new();
    let mut boss_num = 100;
    
    io::stdin().read_line(&mut difficulty).expect("Failed to read input");
    
    match Difficulty::from_input(&difficulty){
        Some(level) => {
            println!("you selected: {}", level.desc());
        }
        None => {
            println!("Enter valid difficulty")
        }
    }
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
       




        println!("Please enter your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed to read input");

        //converting the user input to a number 

        let guess_no: i32= match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid no");
                continue;
            }
        };

        match guess_no.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You won big ff broo");
                break;
            },
            Ordering::Greater => println!("Too big broo try again")
        }

    }

}
