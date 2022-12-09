use std::fs;
use scan_fmt::scan_fmt;

macro_rules! cmp {
    ($x:expr, $y:expr) => {
        {
        let mut ret = 0;
        if $x > $y { ret = 1 }
        else if $y > $x { ret = -1 }
        ret
        }
    };
}

pub fn main(){
    part2();
    part1();
}

fn part1(){
    let con = fs::read_to_string("numbers.txt").unwrap();
    let commands = con.lines().map(|x| scan_fmt!(x, "{} {d}", char, i32).unwrap());
    let mut fields:Vec<Vec<i32>> = vec![vec![0,0]];
    let mut headpos:Vec<i32> = vec![0, 0];//First one is line second is column :)
    let mut tailpos:Vec<i32> = vec![0, 0];
    for command in commands{
        for _ in 0.. command.1{
            match command.0{
                'R' => headpos[0] += 1,
                'L' => headpos[0] -= 1,
                'D' => headpos[1] -= 1,
                'U' => headpos[1] += 1,
                _ => println!("loser"),
            }
            let (x, y) = (tailpos[0], tailpos[1]);
            let (i, j) = (headpos[0], headpos[1]);
            let col = x+cmp!(i, x)==i && y+cmp!(j, y)==j;
            (tailpos[0], tailpos[1]) = (if col {x} else {x+cmp!(i, x)},
                        if col {y} else {y+cmp!(j, y)});
            if !fields.contains(&tailpos){
                fields.push(vec![tailpos[0], tailpos[1]]);
            }
        }
    }
    println!("{}", fields.len());
}

fn part2(){
    let con = fs::read_to_string("numbers.txt").unwrap();
    let commands = con.lines().map(|x| scan_fmt!(x, "{} {d}", char, i32).unwrap());
    let mut fields:Vec<Vec<i32>> = vec![vec![0,0]];
    let mut rope:Vec<Vec<i32>> = vec![vec![0, 0];10];
    for command in commands{
        for _ in 0.. command.1{
            match command.0{
                'R' => rope[0][0] += 1,
                'L' => rope[0][0] -= 1,
                'D' => rope[0][1] -= 1,
                'U' => rope[0][1] += 1,
                _ => println!("loser"),
            }
            for knot in 1.. rope.len(){
                let (x, y) = (rope[knot][0], rope[knot][1]);
                let (i, j) = (rope[knot-1][0], rope[knot-1][1]);
                let col = x+cmp!(i, x)==i && y+cmp!(j, y)==j;
                rope[knot]= vec![if col {x} else {x+cmp!(i, x)},
                            if col {y} else {y+cmp!(j, y)}];
                if knot == 9 && !fields.contains(&rope[9]){
                    fields.push(vec![rope[9][0], rope[9][1]]);
                }
            }
        }
    }
    println!("{}", fields.len());
}
