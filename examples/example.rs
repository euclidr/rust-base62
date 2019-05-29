extern crate base62;

fn main() {
    let num1 = 20190101;
    let code = base62::encode(num1);
    println!("{} represented by base62 is {}.", num1, &code);

    let num2 = base62::decode(&code).unwrap();
    println!("{} decode with base62 get {}.", &code, num2);

    if num1 == num2 {
        println!("num1 and num2 are the same.");
    }
}