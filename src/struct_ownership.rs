// #[derive(Debug, Clone)]
// In order to use `Copy` trait, you have to implement `Clone` trait as well

#[derive(Debug, Clone, Copy)]
struct DougsStruct {
  a: i32,
  b: f64,
  // c: String,  // Error occured due to `copy` trait assumes it to be a primitive type
}

// impl Clone for DougsStruct {
//   fn clone(&self) -> Self {
//     Self{ a: self.a, b: self.b }
//   }
// }

#[allow(unused_variables)]
pub fn run() {
  let var_1 = DougsStruct{ a: 9, b: 10.};
  
  /*
  Error: weird that we think structs are stored in stacks
  */
  // some_procedure(var_1);
  // println!("{:?}", var_1);

  /*
  Success case 1: our struct might as well have thousands of fields to deal with. Therefore it is safe to have it borrowed.
  */

  // some_procedure(&var_1);
  // println!("{:?}", var_1);
  
  /*
  Success case 2: put `clone` next to Debug in line 1 and clone the passing argument `DougsStruct` or you can make custom implementation
  */

  some_procedure(var_1);
  println!("{:?}", var_1);

  /*
  Success case3: put `copy` next to Debug in line 1, this way we can make sure that our struct will behave as if it's one of the primitive types.
  */

}

fn some_procedure(param_a: DougsStruct) {
  println!("{:?}", param_a);
}