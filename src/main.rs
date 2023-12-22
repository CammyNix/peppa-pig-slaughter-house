use std::io;
use std::io::prelude::*;

fn main() {
    println!("welcome to the peppa pig slaughter house game :D");
    println!("you have 3 options for the start, go to the slaughterhouse (slaughterhouse), die (die), go back home (home), or go to disney land (disneyland)");
    println!("type your option below");

    let mut input = String::new();

    io::stdin() 
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let input: String = input.trim().parse().expect("please input an option");

    if input == "home" {
        println!("you have gone home");
        std::process::exit(0);
    }
    else if input == "die" {
        println!("you have died, game over");
        std::process::exit(0);
    }
    else if input == "disneyland" {
        println!("welcome to disney land!");
        println!("you are actually at the slaughter house what will you do now");
        println!("what is your next option");
        println!("you can: sell your kids to the slaughter house (sell) and turn them into bananas or poop (poop)");
        let mut input_two = String::new();

        io::stdin() 
            .read_line(&mut input_two)
            .expect("Failed to read line");
        
        let input_two: String = input_two.trim().parse().expect("please input an option");
        if input_two == "poop" {
            println!("you have shat everywhere and rolled around in your shit, you however, got caught by the gaurds and are now piggy food");
        }
        else if input_two == "sell" {
            println!("you have sold your children to the slaughter house and got 1 billion trillion dollars");
            println!("you win");
            std::process::exit(0);
        }
        else {
            println!("try again");
            std::process::exit(0);
        }
    }
    else if input == "slaughterhouse" {
        println!("you are at the slaughter house now");
        println!("what is your next option");
        println!("you can: sell your kids to the slaughter house (sell) and turn them into bananas or poop (poop)");
        let mut input_two = String::new();

        io::stdin() 
            .read_line(&mut input_two)
            .expect("Failed to read line");
        
        let input_two: String = input_two.trim().parse().expect("please input an option");
        if input_two == "poop" {
            println!("you have shat everywhere and rolled around in your shit, you however, got caught by the gaurds and are now piggy food");
        }
        else if input_two == "sell" {
            println!("you have sold your children to the slaughter house and got 1 billion trillion dollars");
            println!("you win");
            std::process::exit(0);
        }
        else {
            println!("try again");
            std::process::exit(0);
        }
    }
    else {
        println!("not an option try again");
        std::process::exit(0);
    }
}