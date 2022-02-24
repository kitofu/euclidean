use std::io;


fn main() {
    println!("Euclidean Algorithm");

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

    println!("number a is {}", a_number);

        //Bいれてね
    println!("Please input number b:");

    let mut b_number = String::new();

    io::stdin().read_line(&mut b_number)   //←         できてない
        .expect("Failed to read line");

    let b_number: u32 = match b_number.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("number b is {}", b_number);

    if a_number < b_number {
        println!("Must be less than number_a")
        continue;
    }


    break;
    }
}