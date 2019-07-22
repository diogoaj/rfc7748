mod point;

fn main(){
	let p = point::Point{
		x: 2,
		y: 0,
	};

	println!("X: {}, Y: {}", p.x, p.y);
}