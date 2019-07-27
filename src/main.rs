mod crypto;

use rug::Integer;

fn main(){
	println!("Welcome to ecc implementation of Curve25519");

	// Test
	let curve = crypto::Curve25519::new();
	let p1 = &curve.base;
	let p2 = crypto::Point::new(Integer::from("14847277145635483483963372537557091634710985132825781088887140890597596352251".parse::<Integer>().unwrap()),
							    Integer::from("8914613091229147831277935472048643066880067899251840418855181793938505594211".parse::<Integer>().unwrap()));

	println!("{}", curve.point_add(&p1, &p2).gety());
}