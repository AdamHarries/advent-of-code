enum OpCode {
    Add,
    Multiply,
    Finish,
}

fn parse_opcode(o: usize) -> Option<OpCode> {
    match o {
        1 => Some(OpCode::Add),
        2 => Some(OpCode::Multiply),
        99 => Some(OpCode::Finish),
        _ => None,
    }
}
fn show_state(v: &Vec<usize>) -> () {
    print!("[");
    v.iter().for_each(|e| print!("{},", e));
    print!("]\n");
}

// evaluate a computer, given an initial state, and report the value at position
fn eval(initial_state: Vec<usize>) -> Vec<usize> {
    // build a computer from the state
    // clone it so we have total control over the size of/we can resize it etc.
    // initialise the pointer to 0
    let mut c: Vec<usize> = initial_state;
    let mut p: usize = 0;
    loop {
        // read the opcode at p
        let opcode = parse_opcode(c[p]).unwrap();
        match opcode {
            OpCode::Finish => break,
            OpCode::Add => {
                let ix0 = c[p + 1];
                let ix1 = c[p + 2];
                let rx = c[p + 3];
                c[rx] = c[ix0] + c[ix1];
            }
            OpCode::Multiply => {
                let ix0 = c[p + 1];
                let ix1 = c[p + 2];
                let rx = c[p + 3];
                c[rx] = c[ix0] * c[ix1];
            }
            _ => break,
        }
        p += 4;
    }
    c
}

fn test(s: &'static str, input: Vec<usize>, expected: Vec<usize>) -> () {
    println!("{}", s);
    print!("Input:    ");
    show_state(&input);
    print!("Expected: ");
    show_state(&expected);
    let result = eval(input);
    print!("Result:   ");
    show_state(&result);
    assert_eq!(result, expected);
}

fn try_inputs(noun: usize, verb: usize) -> usize {
    // Running the first part:
    let mut initial = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 13, 19, 2, 9, 19, 23, 1, 23, 6, 27,
        1, 13, 27, 31, 1, 31, 10, 35, 1, 9, 35, 39, 1, 39, 9, 43, 2, 6, 43, 47, 1, 47, 5, 51, 2,
        10, 51, 55, 1, 6, 55, 59, 2, 13, 59, 63, 2, 13, 63, 67, 1, 6, 67, 71, 1, 71, 5, 75, 2, 75,
        6, 79, 1, 5, 79, 83, 1, 83, 6, 87, 2, 10, 87, 91, 1, 9, 91, 95, 1, 6, 95, 99, 1, 99, 6,
        103, 2, 103, 9, 107, 2, 107, 10, 111, 1, 5, 111, 115, 1, 115, 6, 119, 2, 6, 119, 123, 1,
        10, 123, 127, 1, 127, 5, 131, 1, 131, 2, 135, 1, 135, 5, 0, 99, 2, 0, 14, 0,
    ];
    initial[1] = noun;
    initial[2] = verb;
    let result = eval(initial);
    result[0]
}

fn main() {
    test(
        "Test 1",
        vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
        vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
    );

    test("Test 2", vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]);
    test("Test 3", vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]);
    test(
        "Test 4",
        vec![2, 4, 4, 5, 99, 0],
        vec![2, 4, 4, 5, 99, 9801],
    );
    test(
        "Test ",
        vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
    );
    for noun in 0..144 {
        for verb in 0..144 {
            let output = try_inputs(noun, verb);
            if output == 19690720 {
                println!("FFFFOUUUND THEM!!!");
                println!("Noun: {}, Verb: {}, Output: {}", noun, verb, output);
                println!("Result: {}", 100 * noun + verb);
            }
        }
    }
}
