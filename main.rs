use std::io;

fn main() {
	println!("vending machine");
	let snacks = ["Polo", "KitKat", "Haribo", "Skittles", "m&m's", "Maltesers"];
	let price = [0.80, 0.85, 2.50, 2.80, 2.90, 2.90];
	println!("Put the money in");
	let mut money:f64=0.00;
	let mut input = String::new();

	io::stdin().read_line(&mut input).expect("Failed to read line");
	money = input.trim().parse().expect("Please type number");

	println!("Amount Inserted : {0:.2}\n",money);
	println!("Make Selection");
	println!("--------------------------");
	let mut idx = 0;
	while idx < 6 {
		println!("{0}.{1}: £{2:.2}",idx, snacks[idx], price[idx]);
		idx+=1;
	}
	println!("--------------------------");
	let mut selected = String::new();
	io::stdin().read_line(&mut selected).expect("Failed to read line");
	let selected: usize = selected.trim().parse().expect("Please enter number");
	let element = snacks[selected];
	println!("selected one is: {element}");

	if price[selected]<=money{
		money-=price[selected];
	}else{
		println!("Short of cash...");
	}
	println!("Take your change: {0:.2}",money);
}
