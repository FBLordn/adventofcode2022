use std::fs;

fn part1(){
    let con = fs::read_to_string("numbers.txt").unwrap();
    let mut used: Vec<char> = Vec::new();
    let mut count = 0;
    for chr in con.chars(){
        let length = used.len();
        if length == 4{
            used.remove(0);
        }
        used.push(chr);
        count += 1;
        if !duplicate_check(&used) && length == 4{
            println!("{count}");
            break;
        }
    }
}

fn main(){
    part2();
    part1();
}

fn duplicate_check(lis: &Vec<char>) -> bool{
    for i in 0..lis.len() { // for each element
      for j in i+1..lis.len() { // for each element after it
        if lis[i] == lis[j] { // compare equality
          return true;
        }
      }
    }
    return false;
  }

fn part2(){
    let con = fs::read_to_string("numbers.txt").unwrap();
    let mut used: Vec<char> = Vec::new();
    let mut count = 0;
    for chr in con.chars(){
        let length = used.len();
        if length == 14{
            used.remove(0);
        }
        used.push(chr);
        count += 1;
        if !duplicate_check(&used) && length == 14{
            println!("{count}");
            break;
        }
    }
}