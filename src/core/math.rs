pub enum Sign {
    Plus,
    Minus,
}

pub fn get_i32_len(n: i32) -> usize {
    let mut l = 1;
    if n < 0 {
        l += 1;
    }

    let mut n = n.abs();
    while n >= 10 {
        l += 1;
        n /= 10;
    }

    l
}
