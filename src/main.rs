use copilottest;
fn main() {
	for i in 0..10 {
		println!("index: {} fib: {}", i, copilottest::fibonacci(i));
	}
}
