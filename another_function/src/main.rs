fn main() {
    // println!("Hello, world!");

    // another_function();
    // another_function(5);
    another_function(5, 'h');
}

// fn another_function() {
// fn another_function(x: i32) {
fn another_function(value: i32, unit_label: char) {
    // println!("Another function.");
    // println!("The value of x is: {x}")
    println!("The measurement is: {value}{unit_label}");
}
