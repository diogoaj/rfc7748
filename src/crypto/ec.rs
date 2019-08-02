use rug::Integer;
use rug::ops::Pow;

// Montgomery Curve
// y^2 = x^3 + 486662x^2 + x mod 2^255 - 19
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
			prime: Integer::from(2).pow(255) - 19, 
		}
	}

	// From RFC 7748 - Elliptic Curves for Security
	// ============================================
	pub fn decode_little_endian(&self, b: &Vec<u8>, bits: u32) -> Integer{
		let mut sum = Integer::from(0);
		for i in 0..(bits+7)/8 {
			sum += Integer::from(b[i as usize]) << 8*i;
		}

		return sum;
	}

	pub fn decode_u_coordinate(&self, u: &mut Vec<u8>, bits: u32) -> Integer{
		if bits % 8 != 0{
			let size = u.len() - 1;
			u[size] &= (1<<(bits%8))-1;
		}
		return self.decode_little_endian(u, bits);
	}

	pub fn encode_u_coordinate(&self, u: &mut Integer, bits: u32) -> Vec<u8>{
		let mut arr = vec![0; (bits as usize + 7)/8];

		let r = u.div_rem_floor_ref(&self.prime);
		let (_, u) = <(Integer, Integer)>::from(r);     

		let u = &Integer::from(u);

		for i in 0..arr.len(){
			arr[i] = (Integer::from(u >> 8*i as u32) & Integer::from(0xff)).to_u8().unwrap();
		}

		return arr;
	}

	pub fn decode_scalar_25519(&self, k: &mut Vec<u8>) -> Integer{
		k[0] &= 248;
		k[31] &= 127;
		k[31] |= 64;

		return self.decode_little_endian(k, 255);
	}

	pub fn cswap(&self, swap: &Integer, x_2: &Integer, x_3: &Integer) -> (Integer, Integer) {
		let mask = Integer::from(0 - swap);

		let dummy = mask & Integer::from(x_2^x_3);
		
		return (Integer::from(x_2^&dummy), Integer::from(x_3^&dummy))
	}

	pub fn scalar_multiply(&self, k: &Vec<u8>, u: &Vec<u8>) -> Vec<u8> {
		let mut k_tmp = k.clone();
		let mut u_tmp = u.clone();

		let kk = self.decode_scalar_25519(&mut k_tmp);
		let uu = self.decode_u_coordinate(&mut u_tmp, 255);

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

			let (tx_2, tx_3) = self.cswap(&swap, &x_2, &x_3);
			let (tz_2, tz_3) = self.cswap(&swap, &z_2, &z_3);

			x_2 = tx_2;
			x_3 = tx_3;
			z_2 = tz_2;
			z_3 = tz_3;

			swap = k_t;

			let two = Integer::from(2);

			let a = Integer::from(&x_2 + &z_2);
			let aa = Integer::from(a.pow_mod_ref(&two, &self.prime).unwrap());
			let b = Integer::from(&x_2 - &z_2);
			let bb = Integer::from(b.pow_mod_ref(&two, &self.prime).unwrap());
			let e = Integer::from(&aa - &bb);
			let c = Integer::from(&x_3 + &z_3);
			let d = Integer::from(&x_3 - &z_3);
			let da = Integer::from(&d * &a);
			let cb = Integer::from(&c * &b);

			x_3 = Integer::from(Integer::from(&da + &cb).pow_mod_ref(&two, &self.prime).unwrap());
			z_3 = &x_1*Integer::from(Integer::from(&da - &cb).pow_mod_ref(&two, &self.prime).unwrap());
			x_2 = Integer::from(&aa * &bb);
			z_2 = &e*Integer::from(&aa + &a24 * &e);
		}

		let (tx_2, _) = self.cswap(&swap, &x_2, &x_3);
		let (tz_2, _) = self.cswap(&swap, &z_2, &z_3);

		x_2 = tx_2;
		z_2 = tz_2;

		let mut res = (x_2 * (z_2.pow_mod(&Integer::from(&p-2), &p).unwrap())) % &self.prime;

		return self.encode_u_coordinate(&mut res, 255);
	}
}
