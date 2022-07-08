use std::io;
struct User{
	name: String,
	account: u64,
	balance: u64,
}

fn main() {
	let mut u1 =  User{
				name: String::from("Alice"),
				account: 1234,
				balance: 100000,
	};
	let mut u2 =  User{
				name: String::from("John"),
				account: 9876,
				balance: 205000,
	};
	let mut money:u64=0;
	let mut input = String::new();

	println!("{0},{1},{2}",u1.name, u1.account, u1.balance);
	println!("{0},{1},{2}\n",u2.name, u2.account, u2.balance);
	println!("from {0} to {1}", u1.name, u2.name);
	
	io::stdin().read_line(&mut input).expect("Failed to read line");
	money = input.trim().parse().expect("Please type number");
	println!();
	if u1.balance<money{
		println!("Not enough balance...");
	}else{
		println!("Transfer Finished");
		u1.balance-=money;
		u2.balance+=money;
		println!("{0},{1},{2}",u1.name, u1.account, u1.balance);
		println!("{0},{1},{2}",u2.name, u2.account, u2.balance);
	}
}
