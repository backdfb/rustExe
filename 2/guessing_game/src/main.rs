extern crate rand; //외부에 의존하는 크레이트가 있음

use std::io;
use std::cmp::Ordering; //Ordering 값은 less, Greater, Equal 로, 두 개의 값을 비교할 때 나올 수 있는 결과
use rand::Rng; //rand 내의 모든것을 호출, Rng는 정수 생성기가 구현한 메서드들을 정의한 trait(반드시 스코프 내에 있어야 한다).

fn main() /* 새함수 선언 */ {
    println!("Guess the member!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // 비번
    
    /* println!("The secret number is: {}", secret_number); */ //비번을 출력해주기 때문에 주석처리함

    loop { // 무한루프
        println!("Please input your guess!");

        let mut guess = String::new(); //값을 변수에 저장, let이 변수 선언

        io::stdin().read_line(&mut guess)
            .expect("Failed to lead line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        // i32 = 32비트 정수, u32 = 32비트의 부호없는 정수
        // parse 메서드는 문자열을 숫자로 파싱한다.

        println!("You guessed: {}", guess); //placeholder를 이용한 출력

        match guess.cmp(&secret_number) { 
            // cmp가 guess와 secret_number를 비교한 결과인 Ordering의 값에 따라 무엇을 할 것인지 결정할 수 있다.
            Ordering::Less    => println!("조금만 더 불러봐!"),
            Ordering::Greater => println!("줄일 필요가 있겠네?!"),
            Ordering::Equal   => {
                println!("You win!");
                break; //정답 입력시 종료                
            }
        }
    }
}
