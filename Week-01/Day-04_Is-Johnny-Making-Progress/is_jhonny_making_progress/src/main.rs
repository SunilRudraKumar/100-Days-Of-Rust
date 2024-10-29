fn main() {
    let miles = vec![3, 4, 1, 2];
    println!("Total progress days: {}", progress_days(&miles));
}

fn progress_days(arr: &[u32]) -> u32 {
    let mut count = 0;

    for i in 1..arr.len() {
        if arr[i - 1] < arr[i] {
            count += 1;
        }
    }

    count
}
