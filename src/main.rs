mod crypto;

fn main(){
	let mut k = hex::decode("0900000000000000000000000000000000000000000000000000000000000000").unwrap();
	let mut u = hex::decode("0900000000000000000000000000000000000000000000000000000000000000").unwrap();
	let mut r;

	// Second iteration test ( 1000 iterations )
	// TODO: implement test module
	for _ in 0..1000{
		let tmp_k = k.clone();
		r = crypto::x25519(&mut k, &mut u);
		u = tmp_k;
		k = crypto::encode_u_coordinate(&mut r, 255);
	}
	
	println!("{}", hex::encode(k));
}