use std::io;
use rand::Rng;


fn main() {
    println!("Euclidean Algorithm");

    let mut root_number = rand::thread_rng().gen_range(0, 1001);
    println!("root_number is:{}", root_number); //秘密の数字は次の通り:{}

    let mut a_number = String::new();
//ループでよき数字入れてもらう
    loop{
    println!("Please input number a:");


    io::stdin().read_line(&mut a_number)   //← よく理解してない
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
//ここで計算したらいいかな？
//a_number をStringで定義したからこの後計算できなくて怒られる u32にキャストしたいい?
    // let mut num: u32 = a_number.trim().parse();
    let mut num: u32 = a_number.trim().parse();


    loop{
        let rem = root_number % num;
        if rem == 0{
            println!("answer = {}", num);
            break;
        }else{
            root_number = num;
            num = rem;
        }

    }

}

//関数とかまだわからない