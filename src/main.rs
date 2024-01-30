use std::process::Command;
use std::thread;
use std::time::Duration;
use std::io::Write; // Import Write trait
use std::io::stdout;

mod buddy;
use buddy::Buddy;

const BUDDY: [&str; 4] = ["    
    ()_ _()
    | O O |
    ( >T< )
   .^`-^-'^.
   `.  ;  .'
   | | | | |
  ((_((|))_))",
    "
    ()_ _()
    | ^ ^ |
    ( >T< )
   .^`-^-'^.
   `.  ;  .'
   | | | | |
  ((_((|))_))"
,
"", 
""];

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

    print!("\n {}: |>",buddy_name);
    for character in input_str.chars() {
        print!("{}", character);
        // Flush the output to ensure immediate printing
        stdout().flush().unwrap();

        if character == ' '{
            thread::sleep(Duration::from_millis(50));
        }
        thread::sleep(Duration::from_millis(100));
    }

    print!("<| \n");
}

fn print_message_with_buddy(message:&str, buddy_number:usize, buddy:&Buddy) {

  clear_terminal(); 

  //let message_length = message.len();
  //let over_under_line = "-".to_string().repeat(message_length+4);
  println!("{}", BUDDY[buddy_number]);

  // println!("{}", over_under_line); 

  print_dialouge(message, &buddy.name);

  // println!("{}", over_under_line); 
}

/*
fn print_options(){
    for n in 0..OPTIONS.len(){
        thread::sleep(Duration::from_millis(200));
        println!("{}) {}", n+1, OPTIONS[n]);
        thread::sleep(Duration::from_millis(200));
    }
}

fn promt_options() -> usize{

    loop{

    }
    0
}
*/

pub fn startup() -> Buddy{
    // Initialize a buddy with name, and some random values
    return Buddy::new(String::from("Buddy"));
}

fn main() {
    // Startup
    let buddy = startup();
    print_message_with_buddy("Hello buddy!",1, &buddy);
    thread::sleep(Duration::from_secs(1));
    print_message_with_buddy("What are we going to do today?", 0, &buddy);
    
}

