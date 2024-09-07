pub trait Draw {
  fn draw(&self);
}


// Why do we nede to use Box?
// - dyn Draw is a trait object, which allows for dynamic dispatch. This means that the exact method implementation to call is determined at runtime rather than at compile time.
//   - Trait objects must be stored behind a pointer because their size is not known at compile time. Box is one such pointer type that provides heap allocation.
// - Ownership and lifetimes: Box provides ownership of the heap-allocated value. When bx is dropped, heap-allocated value is also dropped. 
//   - This ensures that the Screen struct owns its components and manages their lifetimes correctly.
pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw (&self) {
    // code to actually draw a button
  }
}

// Alternative way to define Screen struct
// pub struct Screen<T: Draw> {
//   pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//   T: Draw,
// {
//   pub fn run(&self) {
//       for component in self.components.iter() {
//           component.draw();
//       }
//   }
// }