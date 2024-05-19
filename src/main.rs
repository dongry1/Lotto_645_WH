use rand::Rng;
use std::collections::HashSet;
use std::io;

// printing lotto game, 6 numbers
fn lotto(limit: &u32) {
    
    let mut game = HashSet::with_capacity(6);// 6개의 값을 가짐. hashset 는 유일한 값을 저장함
    let mut itr = 1; // 루프를 위한 변수 :iteration -> itr
    // 문자열을 숫자로 바꾸기 한 방법 : limit.trim().parse::<u32>().unwrap() // String to number <u32>
    // 포인터 *limit = real value of variable
    while itr <= *limit {
        loop {
            if game.len() >= 6 {
                break; // if hashet size is bigger than 6, 숫자 6개 이상이 되면 멈춤
            }
            let mut num = rand::thread_rng(); // Random Number Generating
            game.insert(num.gen_range(1..45)); // game hashset 에 넣기
        }
        println!("{}:{:?}", itr, game); // 게임 번호 : 게임숫자 
        game.clear(); // clearing hashset
        itr = itr + 1;
    }
}

// get string return u32 number
fn _to_unsigned_integer(game: &String) -> u32 {
    match game.trim().parse::<u32>() { // String to number <u32>
        Ok(number) => {
            println!("Your input is verified : {}", number);
            return number;
        }
        Err(e) => {
            println!("Error, Only Positive Number is allowed. {e}");
            return 0;
        }
    }
}

fn main() {
    println!("
    # 645 Lotto.
    # Large number would take a while and use your computing power a lot.
    # This Program will generate 6 number between 1 and 45, Lotto Number.
    # How many games do you want to generate?  ");

    let mut no_of_games = String::new(); // for user input
    io::stdin().read_line(&mut no_of_games).expect("error"); // get user input
    let limit = _to_unsigned_integer(&no_of_games); // change user input to U32
    lotto(&limit); // get the lotto number x limit
}

// 1 game : {11, 6, 7, 12, 18, 27}
// 1 game : {6, 2, 16, 43, 30, 1}
