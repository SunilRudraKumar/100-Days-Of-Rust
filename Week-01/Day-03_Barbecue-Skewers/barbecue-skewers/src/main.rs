fn barbecue_counter(arr: Vec<&str>) -> Vec<u32> {
    let mut nonveg: u32 = 0;
    let mut veg: u32 = 0;

    for item in arr {
        if item.contains("x") { // Check for non-vegetarian indicator
            nonveg += 1;
        } else {
            veg += 1;
        }
    }

    vec![veg, nonveg] // Return as a vector, not an array
}

fn main() {
    let result = barbecue_counter(vec![
        "--oooo-ooo--",
        "--xx--x--xx--",
        "--o---o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--"
    ]);

    println!("{:?}", result); // Display the result
}
