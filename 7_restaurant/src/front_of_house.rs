
// oooo submodules inside
// modules can also hold definitions such as structs, senums, constants, traits, and functions
pub mod hosting { 
  pub fn add_to_waitlist() {}

  fn seat_at_table() {}
}

mod serving {
  fn take_order() {}

  fn serve_order() {}

  fn take_payment() {}
}