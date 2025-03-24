// zadanie 1
fn co_drugi_znak(s: &str) -> String {
    s.chars().step_by(2).collect()
}

// zadanie 2
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn  szyfruj(s: &str, key: usize) -> String {
    if key == 0 {
        return s.to_string();
    }

    let segments = s.chars()
        .collect::<Vec<char>>()
        .chunks(key)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>();

    let mut result = String::new();

    for str in segments {
        result.push_str(reverse_string(&str).as_str());
    }

    result

}

// zadanie 3
fn wizytowka(name: &str, surname: &str) -> String {

    let name_first_letter = name.chars().next().unwrap().to_uppercase();
    let surname_first_letter = surname.chars().next().unwrap().to_uppercase();
    let surname_rest = surname.chars().skip(1).collect::<String>().to_lowercase();

    let mut result = name_first_letter.to_string() + ". ";
    result.push_str(surname_first_letter.to_string().as_str());
    result.push_str(surname_rest.as_str());
    result
}

// zadanie 4

fn na_rzymskie(n: u64) -> String {
    let mut number = n;
    let roman_numerals = [
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")
    ];

    let mut result = String::new();

    for &(value, numeral) in roman_numerals.iter() {
        while number >= value {
            result.push_str(numeral);
            number -= value;
        }
    }
    result
}

fn test_rzymskie() {
    println!("{}", na_rzymskie(3) == "III");
    println!("{}", na_rzymskie(9) == "IX");
    println!("{}", na_rzymskie(19) == "XIX");
    println!("{}", na_rzymskie(1910) == "MCMX");
}

// zadanie 5

fn dodaj_pisemnie(a: &str, b: &str) -> String {
    let mut result: String = String::new();

    let mut index = 0;

    while index < a.len() && index < b.len() {

    }

    result
}

fn main() {
    test_rzymskie();
}
