#[allow(warnings)] // This is so I don't get warnings
pub fn run () {
  // STACK
  // - Fast memory creation and retrieval... Speed, speed, speed!
  // - Memory is automatically recaptured by the program after variable go out of scope
  // - Is the default in Rust
  // - Fixed size variable, allocating memory at compile time ... Collections CANNOT be stack based (and strings are a collections of u8's)
  
  let stack_i8: i8 = 10;
  let stack_f32: f32 = 20.;
  let stack_bool: bool = true;
  let stack_char: char = 'a';

  if stack_i8 == 3 {
    let inside_scope = 9;
    println!("{}", inside_scope);
  }

  //HEAP
  // - Flexibility
  // - Memory that can grow in size (Vector, Hashmap, String, etc)
  // - Runtime performance cost (speed)
  // - Memory that can live beyond the scope that created it
  // - Memory is automatically recaptured when the last OWNER goes out of scope
  let heap_vector: Vec<i8> = Vec::new(); //vec![5,2];
  let heap_string: String = String::from("Howdy");
  let heap_i8: Box<i8> = Box::new(30);

  let stack_i8_2 = stack_i8;
  println!("{}", stack_i8);
  println!("{}", stack_i8_2); // No error because stack will just copy, 
  // stack_i8 and stack_i8_2 "own" different memory
  // rust defaults to Stack allocation whenever it can

  let heap_i8_2 = heap_i8;
  // println!("{}", heap_i8); // Error due to ownership transfer
  println!("{}", heap_i8_2);
}

#[allow(warnings)]
pub fn example() {
  let stack_f64: f64 = 1.;
  let mut heap_f64: Box<f64> = Box::new(2.);
  stack_procedure(stack_f64); // stack_f64 will be copied when passed as an argument
  println!("In main stack {}", stack_f64);

  // heap_procedure_transfer_ownership(heap_f64);
  // println!("In main heal {}", heap_f64); // Error - Memory ownership has transferred from heap_f64 to param argument in `heap_procedure_transfer_ownership`

  // Ugly solution
  // heap_f64 = heap_procedure_1(heap_f64, Box::new(true)).0;  

  // Ideal solution
  heap_procedure_2(&heap_f64); // Ownership stays with heap_f64
  println!("In main heal {}", heap_f64);

}

fn stack_procedure(mut param: f64) { // Mutating won't change what's in example() 
  param += 9.;
  println!("In stack_procedure with param {}", param);
}

#[allow(warnings)]
fn heap_procedure_transfer_ownership(param: Box<f64>) {
  println!("In heap_procedure with param {}", param);
}

// Ugly solution - this is a bad solution to a simple problem because you would have to return every single argument you have passed in inside `heap_procedure_1` function
#[allow(warnings)]
fn heap_procedure_1(param: Box<f64>, param_b: Box<bool>) -> (Box<f64>, Box<bool>) {
  println!("In heap_procedure_1 with param {}", param);
  (param, param_b)
}

// Ideal solution
fn heap_procedure_2(param: &Box<f64>) {
  println!("In heap_procedure with param {}", param);
}