use std::io::stdout;
use std::io::Write; // Import Write trait
use std::process::Command;
use std::thread;
use std::time::Duration;

mod buddy;
use buddy::Buddy;

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

fn print_message_with_buddy(message: &str, buddy: &Buddy) {
    clear_terminal();

    println!("{}", buddy.get_random_image());

    print_dialouge(message, &buddy.name);
}

pub fn startup() -> Buddy {
    // Initialize a buddy with name, and some random values
    return Buddy::new(String::from("Buddy"));
}

fn main() {
    // Startup
    let buddy = startup();
    print_message_with_buddy("Hello buddy!", &buddy);
    thread::sleep(Duration::from_secs(1));

    print_message_with_buddy("What are we going to do today?", &buddy);
    thread::sleep(Duration::from_secs(1));

    print_message_with_buddy("Hmm, what to do...", &buddy);
    thread::sleep(Duration::from_secs(1));

    print_message_with_buddy("Hello? Are you there hooman?", &buddy);
}
