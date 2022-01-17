fn main() {
    let days = ["Sunday", "Monday"];
    let bytes = [0; 5];
    let first = days[0];
    let second = days[1];
    //let third = days[2];

    let three_nums = vec![15, 3, 46];
    println!("Inital vector: {:?}", three_nums);
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    let mut fruit = Vec::new();
    fruit.push("Apple");
    fruit.push("Banana");
    println!("Fruit: {:?}", fruit);
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruit: {:?}", fruit);

    let mut index_vec = vec![1, 3, 5];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);
    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec);
    // let beyond = index_vec[10];
    // println!("{}", beyond);

    let colors = ["Blue", "Green", "Red", "Silver"];
    let mut car: Car;
    let mut engine: Transmission = Transmission::Manual;

    // Order 3 cars, one car for each type of transmission

    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!(
        "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    if 1 == 2 {
        println!("True");
    } else {
        println!("False");
    }
    let formal = true;
    let greeting = if formal { "Good day to you." } else { "Hey" };
    println!("{}", greeting);
    let num = 500;
    let out_of_range: bool;
    if num < 0 {
        out_of_range = true;
    } else if num == 0 {
        out_of_range = true;
    } else if num > 512 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }
    println!("{}", out_of_range);
}

#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

fn car_quality(miles: u32) -> (Age, u32) {
    if miles > 0 {
        return (Age::Used, miles);
    } else {
        return (Age::New, miles);
    }
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    if car_quality(miles).0 == Age::Used {
        if roof {
            println!(
                "Prepare a used car: {:?}, {}, Hard top, {} miles",
                motor, color, miles
            );
        } else {
            println!(
                "Preparing a used car: {:?}, {}, Convertible, {} miles",
                motor, color, miles
            );
        }
    } else {
        if roof {
            println!(
                "Build a new car: {:?}, {}, Hard top, {} miles",
                motor, color, miles
            );
        } else {
            println!(
                "Build a new car: {:?}, {}, Convertible, {} miles",
                motor, color, miles
            );
        }
    }
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: (car_quality(miles)),
    }
}
