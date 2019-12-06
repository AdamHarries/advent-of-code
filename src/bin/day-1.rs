fn main() {
    // Directly include the weights so that we don't
    // have to bother reading them from a file at runtime.
    let bytes = include_bytes!("../../data/day-1/weights.txt");
    // parse them into a string rather than raw bytes.
    let weights = String::from_utf8_lossy(bytes);
    // calculate the fuel requirement by
    let fuel: u32 = weights
        .lines() // splitting the weights into lines.
        .filter_map(|s| s.parse::<u32>().ok()) // parsing each line into a u32
        .map(|w| (w / 3) - 2) // calculating the requirement for the module
        .sum(); //summing the total fuel requirements.
    println!("Requires {} units of fuel", fuel);
}
