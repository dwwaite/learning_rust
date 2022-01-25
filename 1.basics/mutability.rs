fn main() {

    println!("Let's start adding...");

    // Does not work without declaring mutability
    //let sum = 0;
    let mut sum = 0;

    for i in 0..10 {

        sum += i;
        println!("Adding value {}, current total is {}", i, sum);

    }

    println!("Final value is {}", sum);

}