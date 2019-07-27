mod crypto;

use rug::Integer;

fn main(){
	let curve = crypto::Curve25519::new();
	let p1 = &curve.base;

	let res = curve.montgomery_ladder(p1, Integer::from(1337));
	println!("x: {}", res.get_x());
	println!("y: {}", res.get_y())
}