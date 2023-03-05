use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let x: u32 = rng.gen_range(0..255);
    let y: u32 = rng.gen_range(0..255);
    let acc: u32 = 0;

    println!("{} * {} = {}", x, y, x * y);
    println!("Russian Peasant Multiplication = {}", rp_mult(x, y, acc));
    assert_eq!(rp_mult(x, y, acc), x * y)
}

fn rp_mult(x: u32, y: u32, acc: u32) -> u32 {
    match x {
        0 => 0,                                            // x equals 0
        1 => acc + y,                                      // x equals 1
        x if x % 2 == 1 => rp_mult(x / 2, y * 2, acc + y), // x is odd
        _ => rp_mult(x / 2, y * 2, acc),                   // x is even
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn same_as_mult() {
        let mut rng = rand::thread_rng();
        let x: u32 = rng.gen_range(0..255);
        let y: u32 = rng.gen_range(0..255);
        let acc: u32 = 0;

        assert_eq!(rp_mult(x, y, acc), x * y);
    }

    #[test]
    fn return_zero_if_x_is_zero() {
        let mut rng = rand::thread_rng();
        let x: u32 = 0;
        let y: u32 = rng.gen_range(0..255);
        let acc: u32 = 0;

        assert_eq!(rp_mult(x, y, acc), 0);
    }

    #[test]
    fn return_zero_if_y_is_zero() {
        let mut rng = rand::thread_rng();
        let x: u32 = rng.gen_range(0..255);
        let y: u32 = 0;
        let acc: u32 = 0;

        assert_eq!(rp_mult(x, y, acc), 0);
    }
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
