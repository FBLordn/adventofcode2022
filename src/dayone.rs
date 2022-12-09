use std::fs;

fn part1() {
    let contents = fs::read_to_string("numbers.txt").expect("Error");
    let lines = contents.lines();
    let mut var = 0;
    let mut max = 0;

    for s in lines{
        if s == "" {
            var = 0;
        }
        else{
            var += s.trim().parse::<i32>().unwrap();
        }
        if var > max{
            max = var;
        }
    }
    println!("The max is {max}");
}

fn part2(){
    let contents = fs::read_to_string("numbers.txt").expect("Error");
    let lines = contents.lines();
    let mut var = 0;
    let mut max:Vec<i32> = vec![0, 0, 0];
    for s in lines{
        if s == ""{
            changesmallest(&mut max, var);
            var = 0;
        }
        else{
            var += s.trim().parse::<i32>().unwrap();
        }
    }
    changesmallest(&mut max, var);
    for i in 1..3{
        max[0] += max[i];
    }
    println!("{}", max[0]);
}

fn changesmallest(max: &mut Vec<i32>, var: i32){
    let min = *max.iter().min().unwrap();
    for i in 0..3{
        if var>max[i] && max[i]==min{
            max[i] = var;
            break;
        }
    }
}

fn main(){
    part2();
    part1();
}

