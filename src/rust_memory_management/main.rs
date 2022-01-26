fn proccess(input: String) {}

fn caller() {
    let s = String::from("Hello, world!");
    proccess(s.clone());
    proccess(s);
}

fn print_greeting(message: &String) {
    println!("Greeting: {}", message);
}

fn change(message: &mut String) {
    message.push_str("!");
}

fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn erase(_: String) {}

fn main() {
    let mascot = String::from("ferris");
    let ferris = mascot;
    // println!("{}", mascot);
    let mut greeting = String::from("hello");
    let greeting_reference = &greeting;
    println!("Greeting: {}", greeting);
    print_greeting(&greeting);
    print_greeting(&greeting);
    change(&mut greeting);

    let mut value = String::from("hello");
    // let ref1 = &value;
    // let ref2 = &mut value;
    // println!("{}, {}", ref1, ref2);
    // let x;
    // {
    //     let y = 42;
    //     x = &y;
    // }
    // println!("x: {}", x);
    let magic1 = String::from("abracadabra");
    let magic2 = String::from("shazam");
    let result = longest_word(&magic1, &magic2);
    // let result;
    // {
    //     let magic2 = String::from("shazam");
    //     result = longest_word(&magic1, &magic2);
    // }
    println!("The longest magic word is {}", result);

    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    // erase(text);
    println!("{:?}", fox);
    println!("{:?}", dog);

    // Exercise
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}

// TODO: modify only this function.
fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String {
    vector.push(String::from(value));
    vector.get(vector.len() - 1).unwrap()
}
