use std::io;

pub mod menu;

use menu::sandwich::Sandwich;

fn main() {
	let meats = ["Ham", "Tuna", "Seafood", "Veggie Delite", "Egg Mayo", "B.L.T", "Meatball", "Italian B.M.T", "Turkey","Spicy Italian", "Chicken Teriyaki", "Steak & Cheese"];
	let bread_types = ["Wheat", "White", "Hati Italian", "Honey Oat", "Parmesan Oregano", "Flat Bread"];
	let cheese_types = ["American", "Shredded", "Mozzarella"];
	let sauce = ["Ranch", "Mayo", "Sweet Onion", "Sweet Chilli", "Smoke BBQ", "Salt","Black Pepper"];

	let mut meatbase = String::new();
	let mut bread = String::new();
	let mut length:u64 = 0;
	let mut input = String::new();
	let mut cheese = String::new();
	let mut toast = String::new();
	let mut veges = String::new();
	let mut sauce = String::new();
	let mut setmenu = String::new();

    println!("++++++++++++++++ SUBWAY ++++++++++++++++");
	println!("--------- Select your meatbase ---------");
	let mut cnt = 0;
	for m in meats{
		print!("{}\t", m);
		cnt+=1;
		if cnt%3==0{
			print!("\n");
		}
	}
	println!("---");
	io::stdin().read_line(&mut meatbase).expect("Failed to read line");
	meatbase = meatbase.trim().to_string();
	
	println!("\n-------- Select your bread type --------");
	for bt in bread_types{
		println!("{}", bt);
	}
	println!("---");
	io::stdin().read_line(&mut bread).expect("Failed to read line");
	bread = bread.trim().to_string();
	
	println!("\n-- Select your bread length (15/30 cm)--");
	io::stdin().read_line(&mut input).expect("Failed to read line");
	length = input.trim().parse().expect("Please type number");
	let mut my_sub = Sandwich::new_sub(&meatbase,&bread, length);
   
	println!("\n-------- Select your cheese type -------");
	for ct in cheese_types{
		println!("{}", ct);
	}
	println!("---");
	io::stdin().read_line(&mut cheese).expect("Failed to read line");
	cheese = cheese.trim().to_string();
	my_sub.cheese = String::from(cheese);

	println!("\n------- Please Check your order --------");
	println!("{0}cm {1} Sandwich with {2} bread", my_sub.bread_len, my_sub.meatbase, my_sub.bread);
	println!("WITH\n{} cheese", my_sub.cheese);
}
