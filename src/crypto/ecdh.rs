use crate::crypto::ec::Curve25519;
use ring::rand::SystemRandom;
use ring::rand::SecureRandom;

pub struct KeyPair{
	public_key: Vec<u8>,
	private_key: Vec<u8>,
}

impl KeyPair{
	pub fn new(curve: &Curve25519, rng: &SystemRandom) -> KeyPair{
		let u_coordinate = hex::decode("0900000000000000000000000000000000000000000000000000000000000000").unwrap();
		let mut private = vec![0; 32];

		rng.fill(&mut private).unwrap();

		let public = curve.scalar_multiply(&private, &u_coordinate);

		KeyPair{
			private_key: private,
			public_key: public,
		}
	}

	pub fn new_test(curve: &Curve25519, private_key: Vec<u8>) -> KeyPair{
		let u_coordinate = hex::decode("0900000000000000000000000000000000000000000000000000000000000000").unwrap();
		let public = curve.scalar_multiply(&private_key, &u_coordinate);

		KeyPair{
			private_key: private_key,
			public_key: public,
		}
	}

	pub fn get_public_key(&self) -> &Vec<u8>{
		return &self.public_key;
	}

	pub fn get_private_key(&self) -> &Vec<u8>{
		return &self.private_key;
	}

	pub fn dh_exchange(&self, curve: &Curve25519, public_key: &Vec<u8>) -> Vec<u8>{
		return curve.scalar_multiply(self.get_private_key(), public_key);
	}
}