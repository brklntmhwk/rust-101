/* Iterators are lazy and do nothing unless consumed */

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // should be mutable because the next method consumes the iter (changes internal states of it)
    let mut v1_iter = v1.iter();

    // the iter method above produces immutable refs
    // into_iter for an iter that takes ownership of the iterable objs, and iter_mut for an iter that produces mutable refs
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // the sum method calculates the total (taking ownership of the iterable), running the next method
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // the filter method takes a closure and the val will be included only if the closure returns true (just like JavaScript)
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
