use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


enum RockPaperScissors{
    Rock,
    Paper,
    Siscors,
    Nothing,
}


enum WinLooseTie{
    Win,
    Loose,
    tie,
    Bad,
}
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
    let mut score = 0;
    for p in plays{
        print!("{}\n",p);
        score = score + sim_game(p);
        println!("total score {}",score);
        print!("\n");
    }


}

fn sim_game(game_inputs : &str) -> i32 {
    let elf_in = game_inputs.chars().nth(0).unwrap();
    let my_input = game_inputs.chars().nth(2).unwrap();

    let my_input = convert_input_to_wlt(my_input);
    let mut score = 0;
   //print!("the other guy put {} I put {}\n",elf_in,my_input);
    //score = find_score(elf_in, my_input);
    score = score + find_score_wlt(elf_in, my_input);
    print!("score {}\n",score);
    return score;
}

fn find_score(elf_in: char, my_in:RockPaperScissors) -> i32 {
    if elf_in == 'A' {
        match my_in {
            RockPaperScissors::Rock => 4,
            RockPaperScissors::Paper =>  8,
            RockPaperScissors::Siscors => 3,
            RockPaperScissors::Nothing=> 0,
        } 
    }
    else if elf_in =='B'{
        match my_in {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 5 ,
            RockPaperScissors::Siscors => 9,
            RockPaperScissors::Nothing=> 0,

        } 
    }
    else if elf_in == 'C'{
        print!("scocisor\n");
        match my_in {
            RockPaperScissors::Rock => 7,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Siscors => 6,
            RockPaperScissors::Nothing=> 0,
        } 
    }
    else
    {
        0
    }
}

fn conver_input_to_enum(input:char) -> RockPaperScissors
{
    let mut out = RockPaperScissors::Nothing;
    if input == 'A' || input == 'X'{
        print!("I chose Rock\n");
        out = RockPaperScissors::Rock;
    }
    else if input == 'B' || input == 'Y' {
        print!("I chose paper\n");

        out = RockPaperScissors::Paper;
    }
    else if input == 'C' || input == 'Z' {
        print!("I chose scissors\n");

        out = RockPaperScissors::Siscors;
    }
    return out;
}

fn convert_input_to_wlt(input:char) -> WinLooseTie {
    let mut out = WinLooseTie::Bad;
    if input == 'X'{
        out = WinLooseTie::Loose;
    }
    else if input == 'Y' {
        out = WinLooseTie::tie;

    }
    else if input == 'Z' {
        out = WinLooseTie::Win;
    }
    return out;
}

fn find_score_wlt(elf_in: char, my_in:WinLooseTie)-> i32{
    if elf_in == 'A' {
    // elf got a rock
        match my_in {
            WinLooseTie::tie => 4,
            WinLooseTie::Win =>  8,
            WinLooseTie::Loose => 3,
            WinLooseTie::Bad => 0,
        } 
    }
    else if elf_in =='B'{
        // elf chose paper
        match my_in {
            WinLooseTie::Loose => 1,
            WinLooseTie::tie => 5,
            WinLooseTie::Win => 9,
            WinLooseTie::Bad => 0,

        } 
    }
    else if elf_in == 'C'{
        print!("scocisor\n");
        match my_in {
            WinLooseTie::Win => 7,
            WinLooseTie::Loose => 2,
            WinLooseTie::tie => 6,
            WinLooseTie::Bad=> 0,
        } 
    }
    else
    {
        0
    }
}