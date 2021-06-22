#[allow(unused_variables)] // This is so I don't get warnings
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