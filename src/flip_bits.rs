fn flip_bits_32(n: i64) -> i64 {
    // Flip all 64 bits
    let flip = !n;

    // Clear 32 lower and set 32 upper bits
    let mask = (1 << 32) - 1;

    // Clear 32 upper bits of flipped input
    mask & flip
}
