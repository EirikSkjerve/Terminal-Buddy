use std::io::stdout;
use std::io::Write; // Import Write trait
use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::buddy::Buddy;

/// file that handles displaying of buddy, messages
 
fn clear_terminal() {
    if cfg!(target_os = "windows") {
        // For Windows
        Command::new("cmd")
            .arg("/c")
            .arg("cls")
            .status()
            .expect("Failed to clear terminal.");
    }
}

// Prints the message from buddy with delay between each character
fn print_dialouge(input_str: &str, buddy_name: &String) {
    print!("\n {}: |>", buddy_name);
    for character in input_str.chars() {
        print!("{}", character);
        // Flush the output to ensure immediate printing
        stdout().flush().unwrap();

        if character == ' ' {
            thread::sleep(Duration::from_millis(50));
        }
        thread::sleep(Duration::from_millis(100));
    }

    print!("<| \n");
}

/// prints the given message with a new random image of buddy
pub fn display_message_with_buddy(message: &str, buddy: &Buddy, mood: Option<&str>) {
    clear_terminal();

    println!("{}", buddy.get_random_image());

    print_dialouge(message, &buddy.name);
}

/// displays a list of the stats. Unsure if needed
pub fn display_stats_hard(buddy: &Buddy){

}

/// displays a message conveying the mood of buddy. Should use a function from responses.rs
pub fn display_mood(buddy: &Buddy){

}