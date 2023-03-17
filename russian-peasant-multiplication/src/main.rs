use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let x: u32 = rng.gen();
    let y: u32 = rng.gen();

    println!("{} * {} = {}", x, y, x as u64 * y as u64);
    println!("Russian Peasant Multiplication = {}", rp_mult(x, y));
}

fn rp_mult(x: u32, y: u32) -> u64 {
    _rp_mult(x, y as u64, 0)
}

fn _rp_mult(x: u32, y: u64, acc: u64) -> u64 {
    match x {
        0 => 0,                                             // x equals 0
        1 => acc + y,                                       // x equals 1
        x if x % 2 == 1 => _rp_mult(x / 2, y * 2, acc + y), // x is odd
        _ => _rp_mult(x / 2, y * 2, acc),                   // x is even
    }
}

#[test]
fn rp_mult_if_both_are_max() {
    assert_eq!(
        rp_mult(u32::MAX, u32::MAX),
        u32::MAX as u64 * u32::MAX as u64
    );
}

#[test]
fn rp_mult_if_x_is_zero() {
    assert_eq!(rp_mult(0, u32::MAX), 0);
}

#[test]
fn rp_mult_if_x_is_one() {
    assert_eq!(rp_mult(1, u32::MAX), u32::MAX as u64);
}

#[test]
fn rp_mult_if_y_is_zero() {
    assert_eq!(rp_mult(u32::MAX, 0), 0);
}

#[test]
fn rp_mult_if_y_is_one() {
    assert_eq!(rp_mult(u32::MAX, 1), u32::MAX as u64);
}

// fn russ_peas_mult(x: u32, y: u32, acc: u32) -> u32 {
// if x == 1 {
//     acc += y;
//     return acc;
// } else if x % 2 == 1 {
//     acc += y;
// }

// return russ_peas_mult(x / 2, y * 2, acc);
// }
