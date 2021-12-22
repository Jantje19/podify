fn main() {
	println!("PID: {}", std::process::id());
	println!("ENV: {:?}", std::env::vars().collect::<Vec<(String, String)>>());
	println!("CWD: {:?}", std::env::current_dir());

	println!("DIR:");
    for path in std::fs::read_dir("/").unwrap() {
        println!("\tEntry: {:?}", path.unwrap().path())
    }
}