use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let x: i32 = rng.gen_range(0..255);
    let y: i32 = rng.gen_range(0..255);
    let acc: i32 = 0;

    println!("{} * {} = {}", x, y, x * y);
    println!("russ_peas_mult = {}", russ_peas_mult(x, y, acc));
}

fn russ_peas_mult(x: i32, y: i32, acc: i32) -> i32 {
    match x {
        1 => acc + y,
        x if x % 2 == 1 => russ_peas_mult(x / 2, y * 2, acc + y),
        _ => russ_peas_mult(x / 2, y * 2, acc),
    }
    // if x == 1 {
    //     acc += y;
    //     return acc;
    // } else if x % 2 == 1 {
    //     acc += y;
    // }

    // return russ_peas_mult(x / 2, y * 2, acc);
}
