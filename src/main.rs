use std::io::stdout;
use std::io::Write; // Import Write trait
use std::process::Command;
use std::thread;
use std::time::Duration;

mod buddy;
mod display;
mod responses;

use display::display_message_with_buddy;
use buddy::Buddy;

/// main file

/// Initialize a buddy with name, and some random values
pub fn startup() -> Buddy {
    return Buddy::new(String::from("Buddy"));
}

fn main() {
    // Startup
    let buddy = startup();
    display_message_with_buddy("Hello buddy!", &buddy, None);
    thread::sleep(Duration::from_secs(1));

    display_message_with_buddy("What are we going to do today?", &buddy, None);
    thread::sleep(Duration::from_secs(2));

    display_message_with_buddy("Could really go for some fishsticks right now, how about you?", &buddy, None);
    thread::sleep(Duration::from_secs(3));

    display_message_with_buddy("You are a great developer, don't let anyone tell you differently!", &buddy, None);
}
