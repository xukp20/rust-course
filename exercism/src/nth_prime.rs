pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    for i in 2..u32::MAX {
        if is_prime(i) {
            count += 1;
            if count == n {
                return i;
            }
        }
    }

    0
}

fn is_prime(n: u32) -> bool {
    let mut i = 2;
    while i * i < (n + 1) {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }

    true
}