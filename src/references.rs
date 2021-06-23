#[allow(warnings)]
pub fn run() {
  example1();
  example2();
}

fn example1() {
  let var_a = String::from("Howdy!");
  // let mut var_b = &var_a;  // Error: var_b doesn't own var_a
  let var_b = &var_a; 
  let var_c = &var_a;

  println!("{}, {:p}, {:p}", var_a, var_b, var_c);
}

fn example2() {
  let var_a = String::from("Howdy");
  let var_b = String::from("Partner!");

  let mass_data: Vec<&String> = vec![&var_a, &var_b]; // COULD BE MILLIONS OF VALUES

  println!("{}, {}, {:?}", var_a, var_b, mass_data);
}

#[allow(warnings)]
fn heavy_calc(_param: &Vec<&String>) -> i64 {
  //Some heavy duty calcs performed here that utilize available cores of my computer
  14
}