use rug::Integer;

// Curve 25519 used for ECDH

// Montgomery Curve
// y^2 = x^3 + 486662x^2 + x mod 2^252 + 27742317777372353535851937790883648493
pub struct Curve25519{
	pub a: Integer,
	pub b: Integer,
	pub prime: Integer,
}

impl Curve25519{
	pub fn new() -> Curve25519{
		Curve25519{ 
			a: Integer::from(486662), 
			b: Integer::from(1),
			prime: Integer::from(Integer::u_pow_u(2, 255)) - Integer::from(19), 
		}
	}
}


pub struct Point{
	pub x: Integer,
	pub y: Integer,
}
