


fn zadanie1() {
    for i in 33..=126 {
        println!("{}: {}", i, i as u8 as char);
    }
}


fn collatz(number: i32, mut iterator: i32) -> i32 {
    if(number == 1) {
        return iterator;
    }

    iterator += 1;

    if number % 2 == 0 {
        collatz(number / 2, iterator)
    } else {
        collatz(3 * number + 1, iterator)
    }
}

fn zadanie2() {
    let mut number = 12;
    let mut iterator = 0;

    let result = collatz(number, iterator);

    println!("Collatz result is {}", result);
}


fn digit_count(number: i32) -> i32 {
    let mut count = 0;
    let mut number = number;

    while number != 0 {
        count += 1;
        number /= 10;
    }

    count
}

fn is_armstrong(num: i32) -> bool {
    let mut number = num;
    let mut array = [0 ; 10];
    let mut iterator = 0;

    if(num == 0) {
        return true;
    }

    while number != 0 {
        array[iterator] = number % 10;
        number /= 10;
        iterator += 1;
    }

    let mut sum = 0;

    for i in 0..iterator {
        sum += array[i].pow(iterator as u32);
    }

    sum == num
}

fn zadanie3() {
    let numbers = [153, 9474, 9475, 370, 371, 407, 0, 1, 10];

    for num in numbers {
        println!("{}", is_armstrong(num));
    }
}


fn is_perfect_number(number: i32) -> bool {
    let mut sum = 0;

    for i in 1..number {
        if number % i == 0 {
            sum += i;
        }
    }

    sum == number
}

fn zadanie4() {
    let num = 496;

    println!("{}", is_perfect_number(num));
}


fn prime_factorization(num: i32) {
    if(num < 2) {
        println!("brak czynnikow pierwszych dla: {}", num);
        return;
    }

    let mut number = num;

    let mut factor = 2;

    while number > 1 {
        while number % factor == 0 {
            println!("Factor: {}", factor);
            number /= factor;
        }
        factor += 1;
    }

    println!()
}

fn zadanie5() {
    let num = 50;

    prime_factorization(num);
}

fn fast_pow(mut base: u128, mut exponent: u128) -> u128 {
    let mut result = 1;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result *= base;
        }
        base *= base;
        exponent /= 2;
    }

    result
}

fn pow_mod(x: u128, n: u128, p: u128) -> u128 {

    0
}

fn main() {
    //zadanie1();
    //zadanie2();
    //zadanie3();
    //zadanie4();
    //zadanie5();
    println!("Wynik {}", fast_pow(2, 6));
}

