extern crate ecc;
extern crate rug;

#[cfg(test)]

mod tests{
	#[test]
	fn decode_scalar_test() {
		let curve = ecc::crypto::ec::Curve25519::new();
		let scalar1 = "a546e36bf0527c9d3b16154b82465edd62144c0ac1fc5a18506a2244ba449ac4";
		let mut scalar1_bytes = hex::decode(scalar1).unwrap();

		let scalar2 = "4b66e9d4d1b4673c5ad22691957d6af5c11b6421e0ea01d42ca4169e7918ba0d";
		let mut scalar2_bytes = hex::decode(scalar2).unwrap();

		let res1 = 
			rug::Integer::from("31029842492115040904895560451863089656472772604678260265531221036453811406496"
			.parse::<rug::Integer>()
			.unwrap());
		let res2 = 
			rug::Integer::from("35156891815674817266734212754503633747128614016119564763269015315466259359304"
			.parse::<rug::Integer>()
			.unwrap());

	    assert_eq!(curve.decode_scalar_25519(&mut scalar1_bytes), res1);
	    assert_eq!(curve.decode_scalar_25519(&mut scalar2_bytes), res2);
	}

	#[test]
	fn decode_coordinate_test() {
		let curve = ecc::crypto::ec::Curve25519::new();
		let coordinate1 = "e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c";
		let mut coordinate1_bytes = hex::decode(coordinate1).unwrap();

		let coordinate2 = "e5210f12786811d3f4b7959d0538ae2c31dbe7106fc03c3efc4cd549c715a493";
		let mut coordinate2_bytes = hex::decode(coordinate2).unwrap();

		let res1 = 
			rug::Integer::from("34426434033919594451155107781188821651316167215306631574996226621102155684838"
			.parse::<rug::Integer>()
			.unwrap());
		let res2 = 
			rug::Integer::from("8883857351183929894090759386610649319417338800022198945255395922347792736741"
			.parse::<rug::Integer>()
			.unwrap());

	    assert_eq!(curve.decode_u_coordinate(&mut coordinate1_bytes, 255), res1);
	    assert_eq!(curve.decode_u_coordinate(&mut coordinate2_bytes, 255), res2);
	}

	#[test]
	fn scalar_multiply_test(){
		let curve = ecc::crypto::ec::Curve25519::new();
		let scalar1 = "a546e36bf0527c9d3b16154b82465edd62144c0ac1fc5a18506a2244ba449ac4";
		let scalar2 = "4b66e9d4d1b4673c5ad22691957d6af5c11b6421e0ea01d42ca4169e7918ba0d";
		let coordinate1 = "e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c";
		let coordinate2 = "e5210f12786811d3f4b7959d0538ae2c31dbe7106fc03c3efc4cd549c715a493";

		let output1 = "c3da55379de9c6908e94ea4df28d084f32eccf03491c71f754b4075577a28552";
		let output2 = "95cbde9476e8907d7aade45cb4b873f88b595a68799fa152e6f8f7647aac7957";
		let output1_bytes = hex::decode(output1).unwrap(); 
		let output2_bytes = hex::decode(output2).unwrap();

		let mut scalar = hex::decode(scalar1).unwrap();
		let mut coordinate = hex::decode(coordinate1).unwrap();
		assert_eq!(curve.scalar_multiply(&scalar, &coordinate), output1_bytes);

		scalar = hex::decode(scalar2).unwrap();
		coordinate = hex::decode(coordinate2).unwrap();
		assert_eq!(curve.scalar_multiply(&scalar, &coordinate), output2_bytes);
	}

	#[test]
	#[ignore]
	fn scalar_multiply_iteration_test(){
		let curve = ecc::crypto::ec::Curve25519::new();
		let iteration1 = 
			hex::decode("422c8e7a6227d7bca1350b3e2bb7279f7897b87bb6854b783c60e80311ae3079")
			.unwrap();
		let iteration1000 = 
			hex::decode("684cf59ba83309552800ef566f2f4d3c1c3887c49360e3875f2eb94d99532c51")
			.unwrap();
		let iteration1000000 = 
			hex::decode("7c3911e0ab2586fd864497297e575e6f3bc601c0883c30df5f4dd2d24f665424")
			.unwrap();
		
		let mut k = 
			hex::decode("0900000000000000000000000000000000000000000000000000000000000000")
			.unwrap();
		let mut u = k.clone();
		let mut r;

		for i in 0..1000000{
			let tmp_k = k.clone();
			r = curve.scalar_multiply(&k, &u);
			u = tmp_k;
			k = r;
			if i == 0{
				assert_eq!(k, iteration1);
			}else if i == 999{
				assert_eq!(k, iteration1000);
			}else if i == 999999{
				assert_eq!(k, iteration1000000);
			}
		}
	}
}