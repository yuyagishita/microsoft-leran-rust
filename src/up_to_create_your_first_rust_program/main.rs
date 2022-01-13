fn main() {
    println!("This is test{}, test{}", "a", "b");

    let mut a_number;
    let a_word = "Ten";
    a_number = 10;
    println!("The number is {}", a_number);
    println!("The word is {}", a_word);

    a_number = 15;
    println!("Now the number is {}", a_number);

    let shadow_num = 5;
    let shadow_num = shadow_num + 5;
    let shadow_num = shadow_num * 2;
    println!("The number is {}.", shadow_num);

    let number: u32 = 14;
    println!("The number is {}.", number);

    let number_64 = 4.0;
    let number_32: f32 = 5.0;
    println!("number_64 = {}, number_32 = {}", number_64, number_32);
    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1u32 + 2,
        8i32 - 5,
        15 * 3
    );

    // Specify the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';

    // Complier interprets a single item in quotations as the "char" data type
    let smiley_face = '😃';

    // Complier interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    let string_1 = "miley ";

    // Specify the data type "str" with the reference syntax "&str"
    // let string_2: &str = "ace";
    let string_2 = "ace";

    println!(
        "{} is a {}{}{}{}.",
        smiley_face, character_1, string_1, character_2, string_2
    );

    let tuple_e = ("E", 5i32, true);
    println!(
        "Is '{}' the {}th letter of the alpahbet? {}",
        tuple_e.0, tuple_e.1, tuple_e.2
    );

    struct Student {
        name: String,
        level: u8,
        remote: bool,
    }
    struct Grades(char, char, char, char, f32);
    struct Unit;

    let user_1 = Student {
        name: String::from("Consatnce Sharma"),
        remote: true,
        level: 2,
    };
    let user_2 = Student {
        name: String::from("Dyson Tan"),
        level: 255,
        remote: false,
    };
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);
    println!(
        "{}, level {}, Remote: {}, Grades {}, {}, {}, {}. Average: {}",
        user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4
    );
    println!(
        "{}, level {}, Remote: {}, Grades {}, {}, {}, {}. Average: {}",
        user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4
    );
    struct Student2 {
        name: String,
        level: u8,
        remote: bool,
    }
    let user2_1 = Student2 {
        name: String::from("Dyson Tan"),
        level: 254,
        remote: false,
    };
    println!(
        "name {}, level {}, remote {}",
        user2_1.name, user2_1.level, user2_1.remote
    );

    #[derive(Debug)]
    struct KeyPress(String, char);
    #[derive(Debug)]
    struct MouseClick {
        x: i64,
        y: i64,
    }
    #[derive(Debug)]
    enum WebEvent {
        WELoad(bool),
        WEClick(MouseClick),
        WEKeys(KeyPress),
    }
    let click = MouseClick { x: 100, y: 200 };
    let keys = KeyPress(String::from("Command +"), 'N');
    let we_load = WebEvent::WELoad(true);
    let we_click = WebEvent::WEClick(click);
    let we_key = WebEvent::WEKeys(keys);
    println!(
        "\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",
        we_load, we_click, we_key
    );

    let formal = "Formal: Good bye.";
    let casual = "Casual: See you later!";
    goodbye(formal);
    goodbye(casual);

    let num = 25;
    println!("{} divided by 5 = {}", num, divide_by_5(num));

    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!(
        "Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );
    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!(
        "Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );
    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!(
        "Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );
}

fn goodbye(message: &str) {
    println!("\n{}", message);
}

fn divide_by_5(num: u32) -> u32 {
    if num == 0 {
        return 0;
    }
    num / 5
}

struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0,
    }
}
