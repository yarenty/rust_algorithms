pub fn rec_int_mult(x: u64, y: u64) -> u64 {
    let n = get_n(x, y);
    if n == 1 {
        x * y
    } else {
        let half = n / 2;
        let a = x / 10_u64.pow(half);
        let b = x % 10_u64.pow(half);
        let c = y / 10_u64.pow(half);
        let d = y % 10_u64.pow(half);
        let ac = rec_int_mult(a, c);
        let ad = rec_int_mult(a, d);
        let bc = rec_int_mult(b, c);
        let bd = rec_int_mult(b, d);

        10_u64.pow(n) * ac + 10_u64.pow(half) * (ad + bc) + bd
    }
}


fn get_n(x: u64, y: u64) -> u32 {
    let mut n = 1;
    for i in 0..10 {
        if x / 10_u64.pow(i) == 0 {
            n = i;
            break;
        }
    }
    n
}

pub fn karatsuba(x: u64, y: u64) -> u64 {
    let n = get_n(x, y);
    if n <= 1 {
        x * y
    } else {
        let half = n / 2;
        let a = x / 10_u64.pow(half);
        let b = x % 10_u64.pow(half);
        let c = y / 10_u64.pow(half);
        let d = y % 10_u64.pow(half);
        
        let p = a + b;
        let q = c + d;
        
        let ac = karatsuba(a, c);
        let bd = karatsuba(b, d);

        let pq = karatsuba(p, q);

        let abcd = pq - ac - bd;

        10_u64.pow(n) * ac + 10_u64.pow(half) * abcd + bd
    }
}
