fn main() {
	let p:f64 = 210_000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//Amount
	let b = 1.0-(r/100.0);
	let b = f64::powf(b,n);

	let ci = p * b;
	 println!("Compound interest is {}", ci);
}