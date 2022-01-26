fn proccess(input: String) {}

fn caller() {
    let s = String::from("Hello, world!");
    proccess(s);
    proccess(s);
}
fn main() {
    {
        let mascot = String::from("ferris");
        let ferris = mascot;
        // println!("{}", mascot);
    }
}
