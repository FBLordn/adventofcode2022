use std::fs;

fn main(){
    part2();
    part1();
}

fn part1(){
    let con = fs::read_to_string("numbers.txt").unwrap();
    let forest: Vec<&str> = con.split_whitespace().collect();
    let mut row: usize = 0;
    let mut count: i32 = 0;
    loop{
        if row >= forest.len(){
            break;
        }
        let mut pos: usize = 0;
        for _chr in forest[row].chars(){
            if check_horizontal(forest[row].chars().collect(), pos) || check_vertical(&forest, row, pos){
                count += 1;
            }
            pos += 1;
        }
        row += 1;
    }
    println!("{count}");
}

fn part2(){
    let con = fs::read_to_string("numbers.txt").unwrap();
    let forest: Vec<&str> = con.split_whitespace().collect();
    let mut row: usize = 0;
    let mut maxscore: i32 = 0;
    loop{
        if row >= forest.len(){
            break;
        }
        let mut pos: usize = 0;
        for _chr in forest[row].chars(){
            let score = check_left_and_right(forest[row].chars().collect(), pos) * check_up_and_down(&forest, row, pos);
            if score > maxscore{
                maxscore = score;
            }
            pos += 1;
        }
        row += 1;
    }
    println!("{maxscore}");
}


fn check_horizontal(trees: Vec<char>, pos: usize)-> bool{
    let visible;
    let mut maxl: i32 = 0;
    let mut maxr: i32 = 0;
    let tree = to_int(&trees[pos]);
    if pos == 0 || pos == trees.len()-1{
        return true;
    }
    for i in 0 .. trees.len(){
        let current = to_int(&trees[i]);
        if i < pos{
            if maxl < current{
                maxl = current;
            }
        }
        else if i > pos{
            if maxr < current{
                maxr = current;
            }
        }
    }
    if maxr < tree|| maxl < tree{
        visible = true;
    }
    else{
        visible = false;
    }
    return visible;
}

fn check_vertical(forest: &Vec<&str>, row: usize, pos: usize)->bool{
    let visible;
    let mut maxu = 0;
    let mut maxd = 0;
    let tree = to_int(&forest[row].chars().collect::<Vec<char>>()[pos]);
    if row == 0 || row == forest.len()-1{
        return true;
    }
    for line in 0 .. forest.len(){
        let numbers: Vec<char> = forest[line].chars().collect();
        let current = to_int(&numbers[pos]);
        if line < row{
            if maxu < current{
                maxu = current;
            }
        }
        else if line > row{
            if maxd < current{
                maxd = current;
            }
        }
    }
    if maxd < tree || maxu < tree{
        visible = true;
    }
    else{
        visible = false;
    }
    return visible;
}

fn to_int(chr: &char) -> i32{
    return chr.to_string().parse::<i32>().unwrap();
}

fn check_left_and_right(trees: Vec<char>, pos: usize)-> i32{
    let mut countleft: i32 = 0;
    let mut countright: i32 = 0;
    let treesize = to_int(&trees[pos]);
    if pos == 0 || pos == trees.len()-1{
        return 0;
    }
    for i in (0 .. pos).rev(){
        let current = to_int(&trees[i]);
        countleft += 1;
        if treesize <= current{
            break;
        }
    }
    for j in pos+1 .. trees.len(){
        let current = to_int(&trees[j]);
        countright += 1;
        if treesize <= current{
            break;
        }
    }
    return countleft * countright;
}

fn check_up_and_down(forest: &Vec<&str>, row: usize, pos: usize)->i32{
    let mut countup = 0;
    let mut countdown = 0;
    let treesize = to_int(&forest[row].chars().collect::<Vec<char>>()[pos]);
    if row == 0 || row == forest.len()-1{
        return 0;
    }
    for line in (0 .. row).rev(){
        let numbers: Vec<char> = forest[line].chars().collect();
        let current = to_int(&numbers[pos]);
        countup += 1;
        if treesize <= current{  
            break;
        }
    }
    for lin in row+1 .. forest.len(){
        let numbers: Vec<char> = forest[lin].chars().collect();
        let current = to_int(&numbers[pos]);
        countdown += 1;
        if treesize <= current{
            break;
        }
    }
    return countup * countdown;
}