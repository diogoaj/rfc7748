mod crypto;

use rand::Rng;
use rug::Integer;
use rug::rand::RandState;


fn main(){
	let curve = crypto::Curve25519::new();
	let p1 = &curve.base;	

	let arr: [u8; 32] = [233;32];
	println!("{}", crypto::decode_little_endian(&arr, 255));
}