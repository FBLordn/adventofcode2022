use std::collections::HashMap;
use std::fs;

fn part1(){
    let con = fs::read_to_string("numbers.txt").expect("Error");
    let lines = con.lines();
    let mut score = 0;
    for line in lines{
        let choice: Vec<&str> = line.split_whitespace().collect();
        let mychoice = hashish(choice[1]);
        let enemychoice = hashish(choice[0]);
        if mychoice == enemychoice{
            score += 3;
        }
        else if (mychoice + 1)%3 == enemychoice%3{
            
        }
        else{
            score += 6;
        }
        score += mychoice;
    }
    println!("{score}")
}

fn hashish(s: &str)->i32{
    let decisions = HashMap::from([
        ("A", 1),
        ("B", 2),
        ("C", 3),
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);
    match decisions.get(&s){
        Some(&value) => return value, 
        _ => return -1,
    }
}

fn part2(){
    let contents = fs::read_to_string("numbers.txt").expect("Error");
    let lines = contents.lines(); 
    let mut string: i32 = 0; 
    for line in lines{ 
        let lin: Vec<&str>  = line.split_whitespace().collect(); 
        string += hashm(lin[0], lin[1]); 
    } 
    println!("{string}");
}

fn hashm(s: &str, b: &str)->i32{
    if s == "A" && b == "X"{
        return 3;
    }
    else if s == "B" && b == "X"{
        return 1;
    }
    else if s == "C" && b == "X"{
        return 2;
    }
    else if s == "A" && b == "Y"{
        return 4;
    }
    else if s == "B" && b == "Y"{
        return 5;
    }
    else if s == "C" && b == "Y"{
        return 6;
    }
    else if s == "A" && b == "Z"{
        return 8;
    }
    else if s == "B" && b == "Z"{
        return 9;
    }
    else{
        return 7;
    }
}

fn main(){
    part2();
    part1();
}
  