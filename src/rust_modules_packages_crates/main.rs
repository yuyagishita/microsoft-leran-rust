// mod authentication;
// use regex::Regex;

mod car_facotry {
    pub fn build_car() {
        println!("Honk honk!");
    }
}

mod text_processing {

    pub mod letters {
        pub fn count_letters(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_alphabetic()).count()
        }
    }

    pub mod numbers {
        pub fn count_numbers(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_numeric()).count()
        }
    }
}

fn count_letters_and_numbers(text: &str) -> (usize, usize) {
    let number_of_letters = text_processing::letters::count_letters(text);
    let number_of_numbers = text_processing::numbers::count_numbers(text);
    (number_of_letters, number_of_numbers)
}

fn main() {
    // let mut user = authentication::User::new("jeremy", "super-secret");
    // println!("The username is: {}", user.get_username());
    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    // println!("Did our date match? {}", re.is_match("2014-01-01"))
    // car_facotry::build_car();
    assert_eq!(count_letters_and_numbers("221B Baker Street"), (12, 3));
    assert_eq!(count_letters_and_numbers("711 Maple Street"), (11, 3));
    assert_eq!(count_letters_and_numbers("4 Privet Drive"), (11, 1));
}
