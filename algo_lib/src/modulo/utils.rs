pub fn add<const M: u32>(a: u32, b: u32) -> u32 {
    (a + b) % M as u32
}

pub fn sub<const M: u32>(a: u32, b: u32) -> u32 {
    (a + M as u32 - b) % M as u32
}

pub fn mul<const M: u32>(a: u32, b: u32) -> u32 {
    ((a as i64 * b as i64) % (M as i64)) as u32
}

pub fn div<const M: u32>(a: u32, b: u32) -> u32 {
    mul::<M>(a, inverse::<M>(b))
}

pub fn power<const M: u32>(mut x: u32, mut n: u32) -> u32 {
    let mut p = 1;
    while n > 1 {
        if n & 1 == 1 {
            p = mul::<M>(p, x);
        }
        x = mul::<M>(x, x);
        n /= 2;
    }
    if n == 0 {
        1
    } else {
        mul::<M>(p, x)
    }
}

pub fn inverse<const M: u32>(x: u32) -> u32 {
    power::<M>(x, M - 2)
}

pub fn gen_inverses<const M: u32>(n: usize) -> Vec<u32> {
    let mut inv = vec![0; n];
    inv[1] = 1;
    for i in 2..n {
        inv[i] = (M - mul::<M>(inv[(M % i as u32) as usize], M / i as u32)) % M;
    }
    inv
}
