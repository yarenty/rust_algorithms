#![feature(type_ascription)]
mod sort;
mod matrix;


fn main() {
    println!("Hello, world!");
    let o = matrix::multiplication::rec_int_mult(1234,5678);
    let k = matrix::multiplication::karatsuba(1234,5678);
    println!("{}",o);
    println!("{}",k);
    assert_eq!(o, 1234*5678);
    assert_eq!(k, 1234*5678);
    println!("{}", 1234*5678);
}
