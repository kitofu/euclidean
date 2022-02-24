use std::io;
use rand::Rng;


fn main() {
    println!("Euclidean Algorithm");

    let root_number = rand::thread_rng().gen_range(0, 1001);
    println!("root_number is:{}", root_number); //秘密の数字は次の通り:{}


    loop{

        //まず数字Aいれてね
    println!("Please input number a:");

    let mut a_number = String::new();

    io::stdin().read_line(&mut a_number)   //←         できてない
        .expect("Failed to read line");

    let a_number: u32 = match a_number.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    if a_number > root_number {
        println!("Must be less than root_number");
        continue;
    }


    break;
    }
}