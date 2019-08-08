extern crate ecc;
extern crate rug;
extern crate ring;

#[cfg(test)]

mod tests{ 
    #[test]
	pub fn curve25519_to_edwards_test(){
		let curve = ecc::crypto::ec::Curve25519::new();
        let mut u =
			hex::decode("0900000000000000000000000000000000000000000000000000000000000000")
			.unwrap();

        let y = ecc::crypto::utils::convert_mont(&mut u, &curve);
        let correct_y = 
            rug::Integer::from("46316835694926478169428394003475163141307993866256225615783033603165251855960"
            .parse::<rug::Integer>()
			.unwrap());

        assert_eq!(y, correct_y);
	}
}