#[derive(Debug)]
pub struct Sandwich{
	pub meatbase: String,
	pub bread: String,
	pub bread_len: u64,
	pub cheese: String,
	pub toast: bool,
	pub veges: Vec<String>,
	pub sauce: Vec<String>,
	pub setmenu: bool,
}

impl Sandwich{
	pub fn new_sub(meatbase: &str, bread:&str, bread_len : u64) -> Sandwich{
		Sandwich{
			meatbase: String::from(meatbase),
			bread: String::from(bread),
			bread_len: bread_len,
			cheese: String::from("American"),
			toast: true,
			veges: Sandwich::init_veges(),
			sauce: Sandwich::init_sauce(),
			setmenu: false
		}
	}
	fn init_veges() -> Vec<String>{
		vec!["Lettuce".to_string(), "Tomatoes".to_string(),"Cucumbers".to_string(),"Peppers".to_string(),"Onions".to_string(),"Pickles".to_string(),"Olives".to_string(),"Jalapenos".to_string()]
	}
	fn init_sauce() -> Vec<String>{
		Vec::new()
	}
}
