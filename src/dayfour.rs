use std::fs;

fn main(){
    part2();
    part1();
}

fn part1(){
    let con = fs::read_to_string("numbers.txt").expect("Error");
    let lines = con.lines();
    let mut sum = 0;
    for line in lines{
        let elves: Vec<&str> = line.split(&[',', '-'][..]).collect();
        let first = to_int(elves[0]);
        let second = to_int(elves[1]);
        let third = to_int(elves[2]);
        let fourth = to_int(elves[3]);
        if first <= third && second >= fourth{
            sum += 1;
        }
        else if third <= first && fourth >= second{
            sum += 1;
        }
    }
    println!("{sum}");
}

fn to_int(string: &str) -> i32{
    return string.parse::<i32>().unwrap();
}

fn part2(){
    let con = fs::read_to_string("numbers.txt").expect("Error");
    let lines = con.lines();
    let mut sum = 0;
    for line in lines{
        let elves: Vec<&str> = line.split(&[',', '-'][..]).collect();
        let first = to_int(elves[0]);
        let second = to_int(elves[1]);
        let third = to_int(elves[2]);
        let fourth = to_int(elves[3]);
        if first <= third && third <= second{
            sum += 1;
        }
        else if third <= first && first <= fourth{
            sum += 1;
        }
    }
    println!("{sum}");
}
