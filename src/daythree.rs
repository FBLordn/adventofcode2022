use std::fs;

fn part1(){
    let con = fs::read_to_string("numbers.txt").expect("Error");
    let mut sum = 0;
    let backpacks = con.lines();
    for rucksack in backpacks{
        let letters = rucksack.len();
        let half: &str = &rucksack[0.. letters/2];
        let ohalf: &str = &rucksack[letters/2.. letters];
        for chr in half.chars(){
            if ohalf.contains(chr){
                let val = to_int(chr);
                sum += val;
                break;
            }
        }
    }
    println!("{sum}");
}

fn to_int(ch: char) -> i32 {
    let mut val = ch as i32;
    if val > 96 {
        val -= 96;
    } 
    else {
        val -= 38;
    }
    return val;
}

fn main(){
    part2();
    part1();
}

fn part2(){
    let con = fs::read_to_string("numbers.txt").expect("Error");
    let lines: Vec<&str> = con.split("\n").collect();
    let mut sum = 0;
    let mut count = 0;
    loop{
        if count > lines.len()-2{
            break;
        }
        for chr in lines[count].chars(){
            if lines[count+1].contains(chr) && lines[count+2].contains(chr){
                sum += to_int(chr);
                break;
            }
        }
        count += 3;
    }
    println!("{sum}");
}