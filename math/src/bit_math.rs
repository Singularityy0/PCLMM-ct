use primitive_types::U256;

pub fn msb_u8(x:u8)->u32{
    assert!(x!=0);
    7-x.leading_zeros()
}
pub fn lsb_u8(x:u8)->u32{
    assert!(x!=0);
    x.trailing_zeros()
}
pub fn msb_u16(x:u16)->u32{
    assert!(x!=0);
    15-x.leading_zeros()
}
pub fn lsb_u16(x:u16)->u32{
    assert!(x!=0);
    x.trailing_zeros()
}
pub fn msb_u64(x:u64)->u32{
    assert!(x!=0);
    63-x.leading_zeros()
}
pub fn lsb_u64(x:u64)->u32{
    assert!(x!=0);
    x.trailing_zeros()
}
pub fn msb_u128(x:u128)->u32{
    assert!(x!=0);
    127-x.leading_zeros()
}
pub fn lsb_u128(x:u128)->u32{
    assert!(x!=0);
    x.trailing_zeros()
}
pub fn msb_u256(x:U256)->u32{
    assert!(!x.is_zero());
    255-x.leading_zeros()
}
pub fn lsb_u256(x:U256)->u32{
    assert!(!x.is_zero());
    x.trailing_zeros()
}


