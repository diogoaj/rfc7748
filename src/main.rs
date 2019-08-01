mod crypto;

use rug::Integer;

fn main(){
	let curve = crypto::ec::Curve25519::new();
	let mut k = hex::decode("0900000000000000000000000000000000000000000000000000000000000000").unwrap();
	let mut u = hex::decode("0900000000000000000000000000000000000000000000000000000000000000").unwrap();
	let mut r;

	// Second iteration test ( 1000 iterations )
	// TODO: implement test module

	r = curve.scalar_multiply(&mut k, &mut u);
	println!("{}", hex::encode(r));
}