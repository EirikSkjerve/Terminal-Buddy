use std::io::stdout;
use std::io::Write; // Import Write trait
use std::process::Command;
use std::thread;
use std::time::Duration;

mod buddy;
mod display;

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
    display_message_with_buddy("Hello buddy!", &buddy);
    thread::sleep(Duration::from_secs(1));

    display_message_with_buddy("What are we going to do today?", &buddy);
    thread::sleep(Duration::from_secs(1));

    display_message_with_buddy("Hmm, what to do...", &buddy);
    thread::sleep(Duration::from_secs(1));

    display_message_with_buddy("Hello? Are you there hooman?", &buddy);
}
