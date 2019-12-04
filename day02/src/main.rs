fn main() {
    println!("Hello, world!");
    let x = String::from("1,9,10,3,2,3,11,0,99,30,40,50");

    let parts: Vec<&str> = x.split(',').map(|s| s.trim()).collect();

    for chunk in parts.chunks(4) {
        println!("got {}", chunk[0]);
    }
}
