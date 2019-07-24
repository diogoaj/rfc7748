use rug::Integer;
mod crypto;

fn main(){
	let p = crypto::Point{
		x: Integer::from(2),
		y: Integer::from(3),
	};

	println!("X: {}, Y: {}", p.x, p.y);

	let curve = crypto::Curve25519::new();

	println!("Prime number: {}", curve.prime);
}