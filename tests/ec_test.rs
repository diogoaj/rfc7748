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

		let res1 = rug::Integer::from("31029842492115040904895560451863089656472772604678260265531221036453811406496".parse::<rug::Integer>().unwrap());
		let res2 = rug::Integer::from("35156891815674817266734212754503633747128614016119564763269015315466259359304".parse::<rug::Integer>().unwrap());

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

		let res1 = rug::Integer::from("34426434033919594451155107781188821651316167215306631574996226621102155684838".parse::<rug::Integer>().unwrap());
		let res2 = rug::Integer::from("8883857351183929894090759386610649319417338800022198945255395922347792736741".parse::<rug::Integer>().unwrap());

		println!("{:?}", &coordinate2_bytes);
	    assert_eq!(curve.decode_u_coordinate(&mut coordinate1_bytes, 255), res1);
	    assert_eq!(curve.decode_u_coordinate(&mut coordinate2_bytes, 255), res2);
	}
}