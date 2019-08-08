use rug::Integer;
use crate::crypto::ec::Curve25519;

fn u_to_y(u: &Integer, curve: &Curve25519) -> Integer{
    let one = Integer::from(1);
    return Integer::from(u - &one) * (u + one).invert(&curve.prime).unwrap();
}

pub fn convert_mont(u: &mut Vec<u8>, curve: &Curve25519) -> Integer{
    let u_masked = curve.decode_u_coordinate(u, 255);
    let mut y = u_to_y(&u_masked, curve);
    y.set_bit(0, false);
    y
}