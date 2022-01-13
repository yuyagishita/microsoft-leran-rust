fn main() {
    use std::collections::HashMap;
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(
        String::from("Ancient Roman History"),
        String::from("Very accurate."),
    );
    reviews.insert(
        String::from("Cooking with Rhubarb"),
        String::from("Sweet recipes."),
    );
    reviews.insert(
        String::from("Programming in Rust"),
        String::from("Great examples."),
    );

    let book: &str = "Programming in Rust";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));

    // Remove book review
    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    // Confirm book review removed
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));

    /* Exercise: Use a hash map to track orders */
    let mut orders: HashMap<i32, Car> = HashMap::new();
    // Initialize counter variable
    let mut order = 1;
    // Declare a car as mutable "Car" struct
    let mut car: Car;

    // Start with zero miles
    let mut miles = 0;

    for order in 1..12 {
        // Call car_factory to fulfill order
        // Add order <K, V> pair to "orders" hash map
        // Call println! to show order details from the hash map
        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));

        // Reset miles for order variety
        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
    }

    /* Use for, while, and loop expressions */
    loop {
        println!("We loop forever!");
        break;
    }
    let mut counter = 1;
    let stop_loop = loop {
        counter *= 2;
        if counter > 100 {
            break counter;
        }
    };
    println!("Break the loop at counter = {}.", stop_loop);

    counter = 0;
    while counter < 5 {
        println!("We loop a while...");
        counter = counter + 1;
    }

    let big_birds = ["ostrich", "peacock", "stock"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }
    for number in 0..5 {
        println!("{}", number * 2);
    }
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

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Return tuple with car age ("New" or "Used") and mileage
fn car_quality(miles: u32) -> (Age, u32) {
    // Check if car has accumulated miles
    // Return tuple early for Used car
    if miles > 0 {
        return (Age::Used, miles);
    }

    // Return tuple for New car, no need for "return" keyword or semicolon
    (Age::New, miles)
}

// Build "Car" using input arguments
fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Prevent panic: Check color index for colors array, reset as needed
    // Valid color = 1, 2, 3, or 4
    // If color > 4, reduce color to valid index
    let mut color = order as usize;
    while color > 4 {
        color = color - 4;
    }

    // Add variety to orders for motor type and roof type
    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {
        // 3, 6, 9
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {
        // 2, 4, 8, 10
        motor = Transmission::SemiAuto;
        roof = false;
    } // 1, 5, 7, 11

    // Return requested "Car"
    Car {
        color: String::from(colors[(color - 1) as usize]),
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}
