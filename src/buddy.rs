use rand::Rng;

// Struct for buddy. 
/*
Name: name of buddy
hungry: range from 0-10. 10 is very hungry, 0 is full
tired: range from 0-10. 10 is very tired and sleepy, 0 is fully awake
border: range from 0-10. 10 is very bored and buddy wants to do something. 0 is very content.
*/
pub struct Buddy{
    pub name: String,
    hungry: u8,
    tired: u8,
    bored: u8
}

impl Buddy{
    pub fn new(name:String) -> Self{
        let hungry = rand::thread_rng().gen_range(0..6);
        let tired = rand::thread_rng().gen_range(0..6);
        let bored = rand::thread_rng().gen_range(0..6);

        Buddy{name,hungry,tired,bored}

    }

    pub fn get_stats(&self) -> [u8; 3] {
        return [self.hungry, self.tired, self.bored];
    }

    pub fn update_hungry(&mut self, new_value:u8){
        self.hungry = new_value;
    }
    pub fn update_tired(&mut self, new_value:u8){
        self.tired = new_value;
    }
    pub fn update_bored(&mut self, new_value:u8){
        self.bored = new_value;
    }
}

