mod crypto;

use ring::rand::SystemRandom;
use ring::rand::SecureRandom;

fn main(){
	let curve = crypto::ec::Curve25519::new();
	let rng = SystemRandom::new();

	// Test dh exchange

	let alice_kp = crypto::ecdh::KeyPair::new(&curve, &rng);
	let bob_kp = crypto::ecdh::KeyPair::new(&curve, &rng);

	let alice_shared_key = alice_kp.dh_exchange(&curve, bob_kp.get_public_key());
	let bob_shared_key = bob_kp.dh_exchange(&curve, alice_kp.get_public_key());

	println!("{}", alice_shared_key == bob_shared_key);
}