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

// From RFC 7748 - Elliptic Curves for Security
// ============================================
pub fn decode_little_endian(b: &Vec<u8>, bits: u32) -> Integer{
	let mut sum = Integer::from(0);;
	for i in 0..(bits+7)/8 {
		sum += Integer::from(b[i as usize]) << 8*i;
	}

	return sum;
}

pub fn decode_u_coordinate(u: &mut Vec<u8>, bits: u32) -> Integer{
	let size = u.len() - 1;
	if bits % 8 == 0{
		u[size] &= (1<<(bits%8))-1
	}
	return decode_little_endian(u, bits);
}

pub fn encode_u_coordinate(u: &mut Integer, bits: u32) -> Vec<u8>{
	let prime = Integer::from(2).pow(255) - 19;
	let mut arr = vec![0; (bits as usize + 7)/8];

	let r = u.div_rem_floor_ref(&prime);
	let (_, u) = <(Integer, Integer)>::from(r);     

	let u = &Integer::from(u);

	for i in 0..arr.len(){
		arr[i] = (Integer::from(u >> 8*i as u32) & Integer::from(0xff)).to_u8().unwrap();
	}

	return arr;
}

pub fn decode_scalar_25519(k: &mut Vec<u8>) -> Integer{
	k[0] &= 248;
	k[31] &= 127;
	k[31] |= 64;

	return decode_little_endian(k, 255);
}

pub fn cswap(swap: &Integer, x_2: &Integer, x_3: &Integer) -> (Integer, Integer) {
	let mask = Integer::from(0 - swap);

	let dummy = mask & Integer::from(x_2^x_3);

	let xx_2 = Integer::from(x_2^&dummy);
	let xx_3 = Integer::from(x_3^&dummy);
	
	return (xx_2, xx_3)
}

pub fn x25519(k: &mut Vec<u8>, u: &mut Vec<u8>) -> Integer {
	let kk = decode_scalar_25519(k);
	let uu = decode_u_coordinate(u, 255);

	let a24 = Integer::from(121665);  
	let p = Integer::from(2).pow(255) - Integer::from(19);

	let x_1 = uu.clone();
	let mut x_2 = Integer::from(1);
	let mut z_2 = Integer::from(0);
	let mut x_3 = uu.clone();
	let mut z_3 = Integer::from(1);
	let mut swap = Integer::from(0);

	for t in (0..255).rev(){
		let k_t = Integer::from(&kk >> t) & Integer::from(1);

		swap = Integer::from(&swap ^ &k_t);

		let (tx_2, tx_3) = cswap(&swap, &x_2, &x_3);
		let (tz_2, tz_3) = cswap(&swap, &z_2, &z_3);

		x_2 = tx_2;
		x_3 = tx_3;
		z_2 = tz_2;
		z_3 = tz_3;

		swap = k_t;

		let a = Integer::from(&x_2 + &z_2);
		let aa = Integer::from(a.square_ref()) % &p;
		let b = Integer::from(&x_2 - &z_2) ;
		let bb = Integer::from(b.square_ref()) % &p;
		let e = Integer::from(&aa - &bb);
		let c = Integer::from(&x_3 + &z_3);
		let d = Integer::from(&x_3 - &z_3);
		let da = Integer::from(&d * &a);
		let cb = Integer::from(&c * &b);

		x_3 = Integer::from(Integer::from(&da + &cb).square_ref()) % &p;
		z_3 = (&x_1*Integer::from(Integer::from(&da - &cb).square_ref())) % &p;
		x_2 = Integer::from(&aa * &bb);
		z_2 = &e*Integer::from(&aa + &a24 * &e);
	}

	let (tx_2, _) = cswap(&swap, &x_2, &x_3);
	let (tz_2, _) = cswap(&swap, &z_2, &z_3);

	x_2 = tx_2;
	z_2 = tz_2;

	return (x_2 * (z_2.pow_mod(&Integer::from(&p-2), &p).unwrap())) % p;
}