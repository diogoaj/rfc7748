extern crate ecc;
extern crate rug;
extern crate ring;

#[cfg(test)]

mod tests{
	#[test]
	fn generate_keypair_test(){
		let curve = ecc::crypto::ec::Curve25519::new();
		let a = hex::decode("77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a").unwrap();
	 	let b = hex::decode("5dab087e624a8a4b79e17f8b83800ee66f3bb1292618b6fd1c2f8b27ff88e0eb").unwrap();

	 	let public_key_a = hex::decode("8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a").unwrap();
	 	let public_key_b = hex::decode("de9edb7d7b7dc1b4d35b61c2ece435373f8343c85b78674dadfc7e146f882b4f").unwrap();

	 	let a_kp = ecc::crypto::ecdh::KeyPair::new_test(&curve, a);
	 	let b_kp = ecc::crypto::ecdh::KeyPair::new_test(&curve, b);

	 	assert_eq!(*a_kp.get_public_key(), public_key_a);
	 	assert_eq!(*b_kp.get_public_key(), public_key_b);
	}

	#[test]
	fn dh_exchange_test(){
		let curve = ecc::crypto::ec::Curve25519::new();
		let a = hex::decode("77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a").unwrap();
	 	let b = hex::decode("5dab087e624a8a4b79e17f8b83800ee66f3bb1292618b6fd1c2f8b27ff88e0eb").unwrap();

	 	let shared_key = hex::decode("4a5d9d5ba4ce2de1728e3bf480350f25e07e21c947d19e3376f09b3c1e161742").unwrap();

	 	let a_kp = ecc::crypto::ecdh::KeyPair::new_test(&curve, a);
	 	let b_kp = ecc::crypto::ecdh::KeyPair::new_test(&curve, b);

	 	let shared_key_1 = a_kp.dh_exchange(&curve, b_kp.get_public_key());
	 	let shared_key_2 = b_kp.dh_exchange(&curve, a_kp.get_public_key());

	 	assert_eq!(shared_key_1, shared_key);
	 	assert_eq!(shared_key_2, shared_key);
	}

	#[test]
	fn random_dh_exchange_test(){
		let curve = ecc::crypto::ec::Curve25519::new();
		let rng = ring::rand::SystemRandom::new();

		let a_kp = ecc::crypto::ecdh::KeyPair::new(&curve, &rng);
		let b_kp = ecc::crypto::ecdh::KeyPair::new(&curve, &rng);

		let shared_key_1 = a_kp.dh_exchange(&curve, b_kp.get_public_key());
		let shared_key_2 = b_kp.dh_exchange(&curve, a_kp.get_public_key());

		assert_eq!(shared_key_1, shared_key_2);
	}
}