fn main() {
    let t:f64 = 2.0 * 450_000.00;
	let m:f64 = 1_500_000.00;
	let h:f64 = 3.0 * 750_000.00;
	let d:f64 = 3.0 * 2_850_000.00;
	let a:f64 = 250_000.00;
// Average
    let s = t+m+h+d+a;
    let dn:f64 = 2.0 + 1.0 +3.0 +3.0 +1.0;
    let _avg = s/dn;

    println!("The Sum is {}", s);
    println!("The Average is {}", _avg);

}