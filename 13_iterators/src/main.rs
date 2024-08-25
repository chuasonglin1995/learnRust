// into_iter: Consumes the collection, yielding owned values.
// iter: Borrows the collection, yielding references.

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {val}");
    }
    println!("v1: {:?}", v1);

    // ===== Methods that Produce Other Iterators =====
    let v1: Vec<i32> = vec![1, 2, 3];
    // the thing is that map method returns a new iterator that produces the modified item
    // so your original iterator is still available
    // hence he added collect to consumes it all
    // functions like .map()/ .filter() are iterator adaptors, they are methods on iterators that produce a new iterator
    // unlike in javascript where map is a method on arrays
    let v2: Vec<_> = v1.iter()
        .map(|x| x + 1)
        .collect();  // Collect the results into a new vector of i32

    assert_eq!(v2, vec![2, 3, 4]);


}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}


// cannot use iter() here because it would borrow the shoes vector and we would not be able to use it later
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        // ** Need to make iterator mutable to call next() **
        // because .next() consumes the iterator
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        // .sum() will take ownership of the iterator
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
