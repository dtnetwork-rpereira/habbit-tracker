static mut V_TEST: u8 = 255;

fn main() {
    unsafe {
        V_TEST = 240;
        println!("Number: {}", V_TEST);
    }

    let mut first_number: i8 = 20;
    let mut second_number: i8 = 50;

    std::mem::swap(&mut first_number, &mut second_number);

    println!("First number: {}", first_number);
    println!("Second number: {}", second_number);
}
