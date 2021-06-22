#[allow(warnings)]
pub fn run() {
  let some_string: String = String::from("howdy"); // Strings are always on the heap because it can grow in size
  let some_str: &str = "Partner"; //&str is a pointer... to either stack or heap
  // let some_str_2: str = "test"; // Error

  // NOTE: Unless you are passing in a mutable string to a function, it's recommended that you use string slices (&str)
  some_procedure(&some_string, some_str);
  println!("{} {}", some_string, some_str);
}

fn some_procedure(param_a: &String, param_b: &str) {
  println!("{} {}", param_a, param_b);
}