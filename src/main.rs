use std::fs::File;
use std::io::prelude::*;
use std::path::Path;



fn main() {
    let path = Path::new("input.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldnt open {}: {}",display,why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why)=> panic!("could read {}: {}",display,why),
        Ok(_)=> print!("contains:data\n" ),
    }


    let split_sting = s.split("\n");

    let plays = split_sting.collect::<Vec<&str>>();

    for p in plays{
        print!("{}\n",p);
        sim_game(p);
    }


}

fn sim_game(game_inputs : &str) {
    let elf_in = game_inputs.chars().nth(0).unwrap();
    let my_input = game_inputs.chars().nth(2).unwrap();

    print!("the other guy put {} I put {}\n",elf_in,my_input);


}
