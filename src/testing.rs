fn part1(){
    let con = fs::read_to_string("numbers.txt").unwrap();
    let mut used: Vec<char> = Vec::new();
    let mut count = 0;
    for chr in con.chars(){
        if used.len() == 4{
            used.remove(0);
        }
        used.push(chr);
        if dublicate_check(used){
            println!("{count}");
            break
        }
        count += 1;
    }
}

fn dublicate_check(lis: Vec<char>) -> bool{
    let mut num = 4;
    let mut counter = 4;
    loop{
        if counter > num{
            return false;
        }
        else if counter == 0 && num >= 0{
            return true;
        }
        let insp: char = lis[counter-1];
        for chr in lis{
            if chr == insp{
                num -= 1;
            }
        }
        counter -= 1;
    }
}

fn main(){
    part1();
}