fn main() {
	let variable = 10;
	println!("Variable: {variable}");
	let mut mutable_variable= 10;
	mutable_variable = 15;
	println!("MutableVariable: {mutable_variable}");
	const CONSTANT: i32 = 20;
	println!("Constant: {CONSTANT}");
}