fn main() {
    for number in 0..3 {
        println!("The number is {}", number);
    }

    for number in 0..=3 {
        println!("The number is {}", number);
    }

    let mut counter = 0;
    
    loop {
        counter += 1;
        println!("The counter is {}", counter);
        if counter == 3 {
            break;
        }
    }
    println!("After loop");

    // Named loops: ' to name loops

    let mut counter = 0;
    let mut counter2 = 0;

    'first_loop : loop {
        counter += 1;
        println!("First counter is {}", counter);
        if counter > 9 {
            println!("Now entering inner loop");

            loop {
                counter2 += 1;
                println!("The second is {}", counter2);
                if counter2 == 3 {
                    break 'first_loop;
                }
            }
        }
    }

    // While loop
    let mut counter = 0;

    while counter < 3 {
        counter += 1;
        println!("The counter is {}", counter);
    }
}
