use rand::Rng;

fn main() {
    const SIZE: usize = 10;
    let mut rng = rand::thread_rng();
    let mut a: Vec<i32> = vec![];
    for _ in 0..SIZE {
        a.push(rng.gen_range(100..999));
    }
    println!("{:?}", a);
    println!("{:?}", merge_sort(a));
}

fn merge_sort(a: Vec<i32>) -> Vec<i32> {
    let n = a.len();
    match n {
        1 => a.to_vec(),
        _ => {
            let array_one = a[0..(n / 2)].to_vec();
            let array_two = a[(n / 2)..n].to_vec();

            let array_one = merge_sort(array_one);
            let array_two = merge_sort(array_two);

            merge(array_one, array_two)
        }
    }
}

fn merge(mut a: Vec<i32>, mut b: Vec<i32>) -> Vec<i32> {
    let mut c: Vec<i32> = vec![];

    while !a.is_empty() && !b.is_empty() {
        if a[0] > b[0] {
            c.push(b[0]);
            b.remove(0);
        } else {
            c.push(a[0]);
            a.remove(0);
        }
    }

    // At this point either a or b is empty

    while !a.is_empty() {
        c.push(a[0]);
        a.remove(0);
    }

    while !b.is_empty() {
        c.push(b[0]);
        b.remove(0);
    }

    c
}
