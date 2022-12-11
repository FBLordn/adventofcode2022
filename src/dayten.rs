use std::fs;
use std::collections::VecDeque;

fn main(){
    part2();
    part1();
}

fn part1(){
    let con = fs::read_to_string("numbers.txt").unwrap();
    let lines = con.lines();
    let mut cycle = 0;
    let mut x = 1;
    let mut sum = 0;
    for line in lines{
        let command: Vec<&str> = line.split_whitespace().collect();
        cycle += 1;
        if (cycle-20)%40 == 0{
            sum += x*cycle;
        }
        if command[0] != "noop"{
            cycle += 1;
            if (cycle-20)%40 == 0{
                sum += x*cycle;
            }
            x += to_int(command[1]);
        }
        if cycle >= 220{
            break;
        }
    }
    println!("{sum}");
}

fn to_int(string: &str) -> i32{
    return string.parse::<i32>().unwrap();
}

fn part2(){
    let con = fs::read_to_string("numbers.txt").unwrap();
    let lines = con.lines();
    let mut cycle: i32 = 0;
    let mut x: i32 = 1;
    let mut ui: Vec<String> = vec![String::new(); 6];
    let mut tasks: VecDeque<i32> = VecDeque::new();
    for line in lines{
        let command: Vec<&str> = line.split_whitespace().collect();
        let mut row:usize = (cycle / 40) as usize;
        if command[0] != "noop"{
            let ret = match tasks.pop_front() {
                Some(x) => x,
                None => 0,
            };
            tasks.push_back(to_int(command[1]));
            if cycle%40 >= x-1 && cycle%40 <= x+1{
                ui[row].push_str("#");
            }
            else{
                ui[row].push_str(".");
            }
            cycle += 1;
            x += ret;
        }
        row = (cycle / 40) as usize;
        let ret = match tasks.pop_front() {
            Some(x) => x,
            None => 0,
        };
        if cycle%40 >= x-1 && cycle%40 <= x+1{
            ui[row].push_str("#");
        }
        else{
            ui[row].push_str(".");
        }
        cycle += 1;
        x += ret;
    }
    for s in ui{
        println!("{s}");
    } 

}


