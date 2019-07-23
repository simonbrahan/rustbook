use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if !self.values.contains_key(&arg) {
            self.values.insert(arg, (self.calculation)(arg));
        }

        *self.values.get(&arg).unwrap()
    }
}

fn main() {
    let user_spec_val = 10;
    let rand_num = 7;

    generate_workout(user_spec_val, rand_num);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive.value(intensity));
        println!("Next, do {} situps!", expensive.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today!");
    } else {
        println!("Today, run for {} minutes!", expensive.value(intensity));
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|num| num);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(1, v1);
    assert_eq!(2, v2);
}
