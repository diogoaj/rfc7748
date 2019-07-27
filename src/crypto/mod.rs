use rug::Integer;
use rug::ops::Pow;

// Montgomery Curve
// y^2 = x^3 + 486662x^2 + x mod 2^255 - 19
pub struct Curve25519{
	pub a: Integer,
	pub b: Integer,
	pub prime: Integer,
	pub base: Point,
}

#[derive(Clone)]
pub struct Point{
	pub x: Integer,
	pub y: Integer,
}

impl Point{
	pub fn new(x: Integer, y: Integer) -> Point{
		Point{
			x,
			y,
		}
	}

	pub fn getx(&self) -> &Integer{
		return &self.x;
	}

	pub fn gety(&self) -> &Integer{
		return &self.y;
	}
}


impl Curve25519{
	pub fn new() -> Curve25519{
		Curve25519{ 
			a: Integer::from(486662), 
			b: Integer::from(1),
			prime: Integer::from(Integer::u_pow_u(2, 255)) - Integer::from(19), 
			base: Point::new(Integer::from(9), 
							Integer::from("14781619447589544791020593568409986887264606134616475288964881837755586237401".parse::<Integer>().unwrap())),
		}
	}

	// From https://en.wikipedia.org/wiki/Montgomery_curve#Addition
	pub fn point_add(&self, p1: &Point, p2: &Point) -> Point{
		// Calculate x3 coordinate
		let mut num = (Integer::from(p2.getx()*p1.gety()) - p1.getx()*p2.gety()).square();
		let mut den = Integer::from(p1.getx()*p2.getx())*(Integer::from(p2.getx() - p1.getx()).square());
		let mut inv = den.invert(&self.prime).unwrap();

		let mut dividend = num*inv;
		let mut r = dividend.div_rem_floor_ref(&self.prime); // Mod operation
		let (_, x3) = <(Integer, Integer)>::from(r);         // Get remainder

		// Calcualte y3 coordinate
		num = (Integer::from(2*p1.getx() + p2.getx()) + &self.a)*Integer::from(p2.gety() - p1.gety());
		inv = Integer::from(Integer::from(p2.getx() - p1.getx()).invert_ref(&self.prime).unwrap());

		let tmp1 = num*inv;

		num = Integer::from(p2.gety() - p1.gety()).pow(3);
		den = Integer::from(p2.getx() - p1.getx()).pow(3);
		inv = den.invert(&self.prime).unwrap();

		let tmp2 = num*inv;
		dividend = Integer::from(tmp1 - tmp2 - p1.gety());
		r = dividend.div_rem_floor_ref(&self.prime); // Mod operation
		let (_, y3) = <(Integer, Integer)>::from(r); // Get remainder

		return Point::new(x3, y3);		
	}
}