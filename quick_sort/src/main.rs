use rand::Rng;

fn main() {
    const SIZE: isize = 10;
    let mut rng = rand::thread_rng();
    let mut arr: Vec<i32> = vec![];
    for _ in 0..SIZE {
        arr.push(rng.gen_range(100..999));
    }
    println!("{:?}", arr);
    quick_sort(&mut arr);
    println!("{:?}", arr);
}

fn quick_sort(arr: &mut Vec<i32>) {
    _quick_sort(arr, 0, arr.len() as isize - 1)
}

fn _quick_sort(arr: &mut Vec<i32>, low: isize, high: isize) {
    if low < high {
        let pi = _partition(arr, low, high);

        _quick_sort(arr, low, pi - 1);
        _quick_sort(arr, pi + 1, high);
    }
}

fn _partition(arr: &mut Vec<i32>, low: isize, high: isize) -> isize {
    let pivot = arr[high as usize];
    let mut i = low - 1;

    for j in low..high {
        if arr[j as usize] <= pivot {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, high as usize);
    i + 1
}
