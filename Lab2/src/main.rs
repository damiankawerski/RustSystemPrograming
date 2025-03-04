
fn is_leap(year: i32) -> bool {
    if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
        return true;
    }
    return false;
}

fn zadanie1(rok: i32) {
    if is_leap(rok) {
        println!("Rok {} jest przestępny", rok);
    } else {
        println!("Rok {} nie jest przestępny", rok);
    }
}

fn zadanie2(year: i32, month: i32) {
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap(year) {
                29
            } else {
                28
            }
        }
        _ => {
            println!("Nie ma takiego miesiąca");
            return;
        }
    };
    println!("Rok {} ma {} dni w miesiącu {}", year, days, month);
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    return celsius * 1.8 + 32.0;
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.0) / 1.8;
}

fn zadanie3(celsius: f64) {
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{} stopni Celsjusza to {} stopni Fahrenheita", celsius, fahrenheit);
}

fn zadanie4(fahrenheit: f64) {
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{} stopni Fahrenheita to {} stopni Celsjusza", fahrenheit, celsius);
}


struct Time {
    hours: i32,
    minutes: i32,
    seconds: i32,
}

impl Time {
    fn new(hours: i32, minutes: i32, seconds: i32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }

    fn to_seconds(&self) -> i32 {
        return self.hours * 3600 + self.minutes * 60 + self.seconds;
    }

    fn to_time(&self, seconds: i32) -> Time {
        let hours = seconds / 3600;
        let minutes = (seconds - hours * 3600) / 60;
        let seconds = seconds - hours * 3600 - minutes * 60;
        return Time::new(hours, minutes, seconds);
    }
}

fn diffrence_in_time(time1: Time, time2: Time) -> Time {
    let mut diff = Time::new(0, 0, 0);
    let seconds1 = time1.to_seconds();
    let seconds2 = time2.to_seconds();

    let diff_seconds = if seconds1 > seconds2 {
        seconds1 - seconds2
    } else {
        seconds2 - seconds1
    };

    diff = diff.to_time(diff_seconds);


    return diff;
}

fn zadanie5() {
    let time1 = Time::new(12, 30, 15);
    let time2 = Time::new(12, 45, 15);
    let diff = diffrence_in_time(time1, time2);
    println!("Różnica czasu to {} godzin, {} minut i {} sekund", diff.hours, diff.minutes, diff.seconds);
}

fn zadanie6(number: i32) {
    let mut factorial = 1;
    for i in 1..=number {
        factorial *= i;
    }
    println!("Silnia z {} to {}", number, factorial);
}

fn zadanie7(number: i32) {
    let string_number = number.to_string();
    let mut reversed_number: String = "".to_string();
    for i in string_number.chars().rev() {
        reversed_number.push(i);
    }
    println!("Liczba {} odwrotnie to {}", number, reversed_number);
}

fn zadanie7modulo(number: i32) {
    let mut number_copy = number;
    loop {
        let digit = number_copy % 10;
        print!("{}", digit);
        number_copy /= 10;
        if number_copy == 0 {
            break;
        }
    }
}


fn zadanie8(number: i32) {
    let string_number = number.to_string();
    let mut sum = 0;
    for i in string_number.chars() {
        let digit = i.to_digit(10).unwrap();
        sum += digit;
    }
    println!("Suma cyfr liczby {} to {}", number, sum);
}

fn zadanie8modulo(number: i32) {
    let mut number_copy = number;
    let mut sum = 0;
    loop {
        let digit = number_copy % 10;
        sum += digit;
        number_copy /= 10;
        if number_copy == 0 {
            break;
        }
    }
    println!("Suma cyfr liczby {} to {}", number, sum);
}

fn zadanie9(number: i32) {
    for a in 1..number {
        for b in a..number {
            let c_exp = a.pow(2) + b.pow(2);
            let c = (c_exp as f64).sqrt() as i32;
            if c < number && a.pow(2) + b.pow(2) == c.pow(2) && b < c {
                println!("a: {}, b: {}, c: {}", a, b, c);
            }
        }
    }
}

fn main() {
    //zadanie1(2020);
    //zadanie2(2025, 2);
    //zadanie3(20.0);
    //zadanie4(20.0);
    //zadanie5();
    //zadanie6(5);
    //zadanie7(123450);
    //zadanie7modulo(12137);
    //zadanie8(123);
    //zadanie8modulo(123);
    //zadanie9(12);
}



