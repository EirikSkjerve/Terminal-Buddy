use rand::Rng;

// Struct for buddy.
/*
Name: name of buddy
hungry: range from 0-10. 10 is very hungry, 0 is full
tired: range from 0-10. 10 is very tired and sleepy, 0 is fully awake
border: range from 0-10. 10 is very bored and buddy wants to do something. 0 is very content.
*/
pub struct Buddy {
    pub name: String,
    hungry: u8,
    tired: u8,
    bored: u8,
}

/// images for buddy.
/// whitespaces are included for padding
const IMAGE: [&str; 4] = [
    "    
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
          ((_((|))_))",
    "
            ()_ _()
            |  O O|
            (  >T<)
           .^`-^-'^.
           `.  ;  .'
           | | | | |
          ((_((|))_))",
    "
            ()_ _()
            |O O  |
            (>T<  )
           .^`-^-'^.
           `.  ;  .'
           | | | | |
          ((_((|))_))",
];

/**
Implementation of Buddy type
**/
impl Buddy {
    /*
    constructor that takes a string for name, and outputs a randomly initialized buddy instance */
    pub fn new(name: String) -> Self {
        let hungry = rand::thread_rng().gen_range(0..6);
        let tired = rand::thread_rng().gen_range(0..6);
        let bored = rand::thread_rng().gen_range(0..6);

        Buddy {
            name,
            hungry,
            tired,
            bored,
        }
    }

    /// return the stats of the buddy
    pub fn get_stats(&self) -> [u8; 3] {
        return [self.hungry, self.tired, self.bored];
    }

    /// returns a random image of buddy
    pub fn get_random_image(&self) -> &str {
        IMAGE[rand::thread_rng().gen_range(0..4).clone()]
    }

    /// returns an image of buddy that reflects buddys mood
    pub fn get_mood_image(&self, mood:&str) -> &str{
        return "none";
    }

    /// update the 'hungry' field
    pub fn update_hungry(&mut self, new_value: u8) {
        self.hungry = new_value;
    }

    /// update the 'tired' field
    pub fn update_tired(&mut self, new_value: u8) {
        self.tired = new_value;
    }

    /// update the 'bored' field
    pub fn update_bored(&mut self, new_value: u8) {
        self.bored = new_value;
    }
}
