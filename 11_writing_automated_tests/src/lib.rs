// cmd: cargo test
// cmd: cargo test -- --test-threads=1 (if you want to run the tests in serial)
// cmd: cargo test -- --show-output (if you want to see the output of the passing tests too)

// can also ignore some tests by using #[ignore] attribute
// can also filter the tests by using cargo test <test_name> or cargo test <part_of_test_name>
// for structs, can use PartialEq to compare
// Oooo... unit tests usually go in the same files as the code. Hence we nede #[cfg(test)]
// Cool, rust allows me to test private functions too! 

// cargo bench to iterate over the code and check the performance
/**
    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }

**/


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(
            add_two(2), 
            4,
            "Does not return 4, value was `{}`",
            add_two(2)
        );
    }

    #[test]
    fn another() {
       // panic!("Make this test fail");
    }
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests_rectangle {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

// ====== Should panic ======
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1  {
            panic!("Guess value must be between 1 and 100, got {}", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value);
        };

        Guess { value }
    }
}

#[cfg(test)]
mod tests_guess {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")] // <- checks the panic message
    fn greater_than_100() {
        Guess::new(200);
    }
}

// ====== Result<T, E> ======
#[cfg(test)]
mod tests_result_te {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

// ====== Testing Internal Functions ======
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests_internal {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}