pub fn pot(a: u32, b: u32) -> u32 {
    if b == 0 {
        1
    } else {
        a * pot(a, b - 1)
    }
}

pub fn log(a: u32, b: u32) -> Option<u32> {
    if b == 1 {
        Some(0) // a^0 = 1
    } else if b % a != 0 {
        None // não é potência exata
    } else {
        log(a, b / a).map(|exp| exp + 1)
    }
}