mod crypto;

use rand::Rng;
use rug::Integer;
use rug::rand::RandState;


fn main(){
	let curve = crypto::Curve25519::new();
	let p1 = &curve.base;	

	// Testing multiplication with random 256 bit number

	// Seed rng
	let mut rng = rand::thread_rng();
	let n1: u128 = rng.gen();

	let mut rand = RandState::new();
	rand.seed(&Integer::from(n1));

	// Get random 256 bit number
	let d = Integer::from(Integer::random_bits(256, &mut rand));

	println!("Random number: {}", d);

	// Multiply
	let res = curve.montgomery_ladder(p1, Integer::from(d));
	println!("x: {}", res.get_x());
	println!("y: {}", res.get_y());
}