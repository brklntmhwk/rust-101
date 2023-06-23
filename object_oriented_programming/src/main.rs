/* Three frequently picked up OOP features */
// 1. Objects and their methods (or operations)
// 2. Encapsulation
// 3. Inheritance

/* How Rust approaches to each of them */
// 1. structs, enums, and impl blocks
// 2. the pub marker
// 3. generics and traits (these seem way better in terms of performance, efficiency, and conciseness)

pub struct AveragedCollection {
    // both the list and average are private (encapsulated) to ensure that they are changeable via those calculation methods: add, remove, and average
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    // encapsulate the update_average func since it's not supposed to be used publicly and therefore shouldn't be accessible from the user's standpoint
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {}
