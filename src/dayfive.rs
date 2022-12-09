use std::fs;
use itertools::Itertools;
use scan_fmt::scan_fmt;

pub fn main(){
    part2();
    println!();
    part1();
}

fn part1(){
    let con = fs::read_to_string("numbers.txt").expect("Error");
    let (mut cratelist, instructions) = parse(&con);
    let commands = instructions.lines();
    for line in commands{
        let numbers: Vec<&str> = line.split_whitespace().collect(); //1, 3, 5
        let mut count = 0;
        loop{
            if count == to_int(&numbers[1]){
                break;
            }
            let topcrate:char = cratelist[(to_int(&numbers[3])-1) as usize].pop().unwrap();
            cratelist[(to_int(&numbers[5])-1) as usize].push(topcrate);
            count += 1;
        }
    }
    let mut tops: String = "".to_string();
    for row in cratelist{
        let length = row.len();
        tops.push(row[length-1]);
    }
    println!("{tops}");
}

fn to_int(string: &str) -> i32{
    return string.parse::<i32>().unwrap();
}

fn parse(input: &str) -> (Vec<Vec<char>>, &str) {
    let (crates, instrs) = match input.split("\n\n").into_iter().collect_tuple() {
        Some(x) => x,
        None => input.split("\n\r\n").into_iter().collect_tuple().unwrap(),
    };
    let lines = crates.split('\n').collect::<Vec<&str>>().into_iter().rev().collect::<Vec<&str>>();
    let mut stacklist: Vec<Vec<char>> = vec![Vec::new();lines[0].len()/4];

    for row in lines[1..].into_iter() {
        for (i, item) in row.chars().skip(1).step_by(4).enumerate() {
            if item.is_alphabetic() {
                stacklist[i].push(item);
            }
        }
    }
    return (stacklist, instrs);
}

fn part2(){
    let con = fs::read_to_string("numbers.txt").expect("Error");
    let (mut cratelist, instrs) = parse(&con);
    for instr in instrs.lines() {
        let (mut num, src, dest) = scan_fmt!(instr, "move {d} from {d} to {d}", i32, usize, usize).unwrap();
        let mut storage: Vec<char> = Vec::new();
        loop{
            if 0 >= num{
                break;
            }
            let topcrate:char = cratelist[src-1].pop().unwrap();
            storage.push(topcrate);
            num -= 1;
        }
        for stuff in storage.into_iter().rev(){
            cratelist[dest-1].push(stuff);
        }
    }
    for v in cratelist{ print!("{}", v[v.len()-1]); }
}