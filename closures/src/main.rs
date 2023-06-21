/* Closures can capture values from their envs in three ways:
borrowing immutably, borrowing mutably, and taking ownership */

/* Traits the closure implements, depending on the ways it captures and handles values
1. FnOnce applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once. (e.g. Option's unwrap_or_else)
2. FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once. (e.g. slice's sort_by_key)
3. Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently. */

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // "||" is a closure that could take params in between itself
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    /* borrowing immutably */
    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);
        let only_borrows = || println!("From closure: {:?}", list);
        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
    }
    /* borrowing mutably */
    {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let mut borrows_mutably = || list.push(7);

        borrows_mutably();
        println!("After calling closure: {:?}", list);
    }
    /* taking ownership */
    {
        use std::thread;
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        // the move keyword defines that the ownership of the values be moved
        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();
        // Error: borrow of moved value: `list` value borrowed here after move
        // println!("After calling closure: {:?}", list);
    }

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
