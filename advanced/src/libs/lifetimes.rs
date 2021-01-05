pub fn pass() {
    let x = 5; //start 'b
    let r = &x; //start 'a
    println!("r: {}", r);
    //end 'a

    //since b lives longer than a its valid
} //end 'b

fn longest<'b>(x: &'b str, y: &'b str) -> &'b str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
pub fn lifeMain() {
    let string1 = String::from("long string is long");
    let result;
    let string2 = String::from("xyz");
    {
        // let string2 = String::from("xyz");
        //if same lifetime for two params, rust takes youngest one for both
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

//fails because lifetime of a > b, and  refrence to
// lifetime b made from a, a lives longer than b
//cant reference will lead to referencing bad memeory

// pub fn fail() {
//     let x; //'a
//     {
//         let r = 5; //start 'b
//         x = &r;
//     } //end 'b
//     println!("x: {}", x);
// } //end 'a
