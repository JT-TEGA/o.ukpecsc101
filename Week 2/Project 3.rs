use std::process::Command;

fn main()
{

	let loan:f64 = 210,000.0;
	let rate:f64 = 5.0/100.0;
	let time:f64 = 3.0;
	let mut amount:f64 = 1.0+rate;

	amount = 1.0 + f64::powf(amount,time);
	//println!("{}",f64::powf(2.0,4.0));

	amount = amount * loan;

	let compound_interest:f64 = amount - loan;


	println!("Amount: {}",compound_interest);
	let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}