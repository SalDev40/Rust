

use std::io;
use rand::Rng;

fn guess(){
    let secretNumber = rand::thread_rng().gen_range(1,10);
    let mut yourGuess = String::new();
    
    println!("Secret Number: {}", secretNumber);
    println!("Guess the number");
    io::stdin()
    .read_line(&mut yourGuess)
    .expect("Failed to get input");    

    let guess: u32 = yourGuess.trim()
    .parse().expect("Please type a number!");

    println!("Your Guess: {}", yourGuess);
    if(guess == secretNumber){
        println!("Numbers match!!")
    }
}

fn types(){
    let x : (i32,i32,i32) = (1,2,3);
    println!("tuple first value: {}", x.0);

    let ar = [1,2,3,4,5];
    for i in &ar {
        print!("{} ", i);
    }
}

fn takeAndReturnOwnership(x: String) -> String {
    x
}

fn borrow(x: &String){
    println!("borrowed {} from main", x);
}


fn ownerShip(){
    let z = 5;
    let y = z;

    println!("{}, {}", z, y);

    let one : String  = "apple".to_string();
    println!("before {}", one);
    let two = &one;
    println!("after {}, one no longer owns string", two);


}

// fn dangleReference() -> &String {
//     let x: String =  String::from("Hello world");
//     return &x;
// }

fn slices(x: &mut str){
    println!("slices {}", x);

}

fn main(){
    let mut y = "test".to_string();
    y = takeAndReturnOwnership(y);
    println!("{}", y);

    borrow(&y);

    // println!("dangle Ref {}", dangleReference());

    slices(&mut y[0..]);

}



