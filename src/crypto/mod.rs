use rug::Integer;
use rug::ops::Pow;
use std::cmp::Ordering;

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

	pub fn get_x(&self) -> &Integer{
		return &self.x;
	}

	pub fn get_y(&self) -> &Integer{
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
		if p1.get_x().cmp0() == Ordering::Equal &&
		   p1.get_y().cmp0() == Ordering::Equal{
		   return Point::new(Integer::from(p2.get_x()), Integer::from(p2.get_y()));
		}
	   else if p2.get_x().cmp0() == Ordering::Equal &&
	           p2.get_y().cmp0() == Ordering::Equal{
		   return Point::new(Integer::from(p1.get_x()), Integer::from(p1.get_y()));
		}

		// Calculate x3 coordinate
		let mut num = Integer::from(p2.get_y() - p1.get_y()).square();
		let mut den = Integer::from(p2.get_x() - p1.get_x()).square();
		let mut inv = den.invert(&self.prime).unwrap();

		let mut dividend = num*inv;
		dividend = dividend - &self.a - p1.get_x() - p2.get_x();
		let mut r = dividend.div_rem_floor_ref(&self.prime); // Mod operation
		let (_, x3) = <(Integer, Integer)>::from(r);         // Get remainder

		// Calcualte y3 coordinate
		num = (Integer::from(2*p1.get_x() + p2.get_x()) + &self.a)*Integer::from(p2.get_y() - p1.get_y());
		inv = Integer::from(Integer::from(p2.get_x() - p1.get_x()).invert_ref(&self.prime).unwrap());

		let tmp1 = num*inv;

		num = Integer::from(p2.get_y() - p1.get_y()).pow(3);
		den = Integer::from(p2.get_x() - p1.get_x()).pow(3);
		inv = den.invert(&self.prime).unwrap();

		let tmp2 = num*inv;
		dividend = Integer::from(tmp1 - tmp2 - p1.get_y());
		r = dividend.div_rem_floor_ref(&self.prime); // Mod operation
		let (_, y3) = <(Integer, Integer)>::from(r); // Get remainder

		return Point::new(x3, y3);		
	}

	// From https://en.wikipedia.org/wiki/Montgomery_curve#Doubling
	pub fn point_double(&self, p: &Point) -> Point{
		if p.get_x().cmp0() == Ordering::Equal &&
		   p.get_y().cmp0() == Ordering::Equal{
		   	return Point::new(Integer::from(0), Integer::from(0));
		   }

		// Calculate slope l
		let num = 3*Integer::from(p.get_x().pow(2)) + 
			          Integer::from(2*&self.a)*p.get_x() + 
			          1;

		let den = Integer::from(2*p.get_y());
		let inv = den.invert(&self.prime).unwrap();
		let mut dividend = Integer::from(num*inv);

		let mut r = dividend.div_rem_floor_ref(&self.prime);
		let (_, l) = <(Integer, Integer)>::from(r);

		let slope = &l;

		// Calculate x3 coordinate
		dividend = Integer::from(slope.pow(2)) - 
		           &self.a - 
		           2*p.get_x();

		r = dividend.div_rem_floor_ref(&self.prime);
		let (_, x3) = <(Integer, Integer)>::from(r);

		// Calculate y3 coordinate
		dividend = Integer::from(3*p.get_x() + &self.a)*slope - 
		           Integer::from(slope.pow(3)) - 
		           p.get_y();

		r = dividend.div_rem_floor_ref(&self.prime);
		let (_, y3) = <(Integer, Integer)>::from(r);

		return Point::new(x3, y3);
	}

	// From https://en.wikipedia.org/wiki/Elliptic_curve_point_multiplication#Montgomery_ladder
	pub fn montgomery_ladder(&self, p: &Point, m: Integer) -> Point{
		let mut r0 = &Point::new(Integer::from(0), Integer::from(0));
		let mut r1 = p;
		let mut tmp3;
		let mut tmp4;

		let bits = m.significant_bits() - 1;
		let mut counter = bits as i32;

		while counter >= 0{
			if m.get_bit(counter as u32) == false{
				tmp3 = self.point_add(&r0, &r1);
				r1 = &tmp3;
				tmp4 = self.point_double(&r0);
				r0 = &tmp4;
			}else{
				tmp4 = self.point_add(&r0, &r1);
				r0 = &tmp4;
				tmp3 = self.point_double(&r1);
				r1 = &tmp3;
			}
			counter = counter - 1;
		}

		return Point::new(Integer::from(r0.get_x()), Integer::from(r0.get_y()));
	}
}