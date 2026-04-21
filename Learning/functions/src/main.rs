fn main() {
    let x = five();
    let p_one = plus_one(5);

    println!("The value of x is: {}", x);
    println!("The value of p_one is: {}", p_one);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}


fn five() -> i32 {
    5 // this is a statement, not an expression
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }
