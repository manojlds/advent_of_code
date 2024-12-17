use super::utils::read_input;

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let (mut computer, program) = read_input().expect("Input not found");

    let mut value: usize = 0;
    let mut times = 1;
    loop {
        computer.set_register('A', value);
        computer.set_register('B', 0);
        computer.set_register('C', 0);

        let output = computer.run(program.clone());

        if output == program {
            break;
        }
        if output[(output.len() - times)..] == program[(program.len() - times)..] {
            times += 1;
            value = value * 8;
            
        } else {
            value = value + 1
        }
    }

    println!("Day 17 Part 2: {:?}", value);
    
    Ok(())
}