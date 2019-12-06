fn cfuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn rec_fuel(mass: i32) -> i32 {
    let direct_fuel = (mass / 3) - 2;
    if direct_fuel < 0 {
        0
    } else {
        direct_fuel + rec_fuel(direct_fuel)
    }
}

fn main() {
    // Directly include the weights so that we don't
    // have to bother reading them from a file at runtime.
    let bytes = include_bytes!("../data/module-masses.txt");
    // parse them into a string rather than raw bytes.
    let weights = String::from_utf8_lossy(bytes);
    // calculate the fuel requirement by:
    let total_fuel: i32 = weights
        .lines() // splitting the weights into lines.
        .filter_map(|s| s.parse::<i32>().ok()) // parsing each line into a i32
        .map(rec_fuel) // calculating the requirement for the module
        .sum(); //summing the total fuel requirements.

    println!("Total fuel: {}", total_fuel);

    let test_case1 = rec_fuel(100756);
    println!("Test case 1: {}", test_case1);

    let test_case2 = rec_fuel(1969);
    println!("Test case 1: {}", test_case2);
}
