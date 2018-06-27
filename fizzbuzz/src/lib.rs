pub fn fizzbuzz(bound: usize) {
    for i in 0..bound {
        if i % 3 == 0 {
            if i % 5 == 0 {
                println!("fizzbuzz");
            } else {
                println!("fizz");
            }
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
}
