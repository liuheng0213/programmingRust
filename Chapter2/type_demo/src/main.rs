fn main() {
    /**
         * They make your code:

    More readable

    Easier to refactor

    Less repetitive
         */
    println!("Hello, world!");
    type Kilometers = i32;

    let x: i32 = 10;
    let y: Kilometers = 20;

    println!("x + y = {}", x + y);

    type MathOp = fn(i32, i32) -> i32;

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let op: MathOp = add;
    println!("Result: {}", op(2, 3));
}
