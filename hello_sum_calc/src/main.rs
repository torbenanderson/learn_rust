fn main() {
    let foo: i32 = 5;
    let bar: i32 = 10; // This should show unused variable warning
    let sum: i32 = foo + bar;
    println!(
        "
    {foo} + {bar} = {sum}
    "
    );
}
