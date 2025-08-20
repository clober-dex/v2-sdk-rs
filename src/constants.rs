use alloy::primitives::U256;

pub const PRICE_PRECISION: u32 = 96;
pub const MAX_TICK: i128 = (1i128 << 19) - 1;
pub const MIN_TICK: i128 = -MAX_TICK;
pub const MIN_PRICE_RAW: u128 = 1_350_587;

#[inline]
pub fn min_price() -> U256 {
    U256::from(MIN_PRICE_RAW)
}

#[inline]
pub fn max_price() -> U256 {
    // MAX_PRICE = 4647684107270898330752324302845848816923571339324334
    // keep as a function to avoid const-init issues
    // little-endian limbs: [lo, mid1, mid2, hi]
    U256::from_limbs([
        0x99d8_4952_a89f_cff6,
        0x9486_a4f2_8ec0_40f2,
        0x0990_dba5_75a2_7f4e,
        0x0000_0000_0000_0000,
    ])
}

// helpers for Q96/Q192
#[inline]
pub fn q96_one() -> U256 {
    U256::from(1u8) << PRICE_PRECISION
}

#[inline]
pub fn q192_one() -> U256 {
    U256::from(1u8) << (PRICE_PRECISION * 2)
}
