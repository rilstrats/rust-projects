use rand::Rng;

fn main() {
    const MIN: i32 = 100;
    const MAX: i32 = 999;
    const ITEMS: i32 = 100;
    // let l: Vec<i32> =
    let mut rng = rand::thread_rng();
    let l: Vec<i32> = (0..ITEMS).map(|_| rng.gen_range(MIN, MAX)).collect();

    for (unsort_elm, unsort_i) in l.iter().enumerate() {
        let shift_i = unsort_i - 1;
        while shift_i >= 0 && l[shift_i] > unsort_elm {}
    }
}
