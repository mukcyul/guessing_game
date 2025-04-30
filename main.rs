//"Libraries"
use std::{
    io,
    cmp::Ordering,
    thread,
    time
};
use rand::Rng;

fn main() {
    //Versuchen
    let mut attempts = 10;

    println!("Guess the number!");

    //Erhalten wir unsere zufällige Geheimnummer 
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is: {secret_number}");

    //Wann wir haben mehr als 0 Versuchen
    while attempts > 0 {
        println!("Please input your guess (1-100): ");
        
        //Warten auf Benutzereingaben
        let mut guess = String::new();

        //Fehlerprüfung
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //String in Integer übertragen
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        //Übereinstimmende erratene und geheime Nummer
        match guess.cmp(&secret_number) {
            //Nummer sind weniger?
            Ordering::Less => {
                println!("Too small");
                attempts -= 1;
                println!("Attempts: {}", attempts);
            }
            // Nummer sind großer?
            Ordering::Greater => {
                println!("Too big");
                attempts -= 1;
                println!("Attempts: {}", attempts);
            }
            //Hat der Benutzer richtig geraten?
            Ordering::Equal => {
                println!("You win, closing the game...");
                break;
            }
        }
    //Wenn der Benutzer hat 0 Versuchen - das Spiel ist verloren und
    //das Programm sagt die richtige Anzahl
    if attempts == 0 {
            println!("You lost! The number was {secret_number}.");
        }
    }
    thread::sleep(time::Duration::from_secs(3));
}