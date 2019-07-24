use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<F, K, V> {
    calculation: F,
    values: HashMap<K, V>,
}

impl<F, K, V> Cacher<F, K, V>
where
    F: Fn(&K) -> V,
    K: Hash + Eq,
    V: Clone,
{
    fn new(calculation: F) -> Self {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        match self.values.get(&arg) {
            Some(val) => val.clone(),
            None => {
                let val = (self.calculation)(&arg);
                self.values.insert(arg, val.clone());
                val
            }
        }
    }
}

fn main() {
    let user_spec_val = 10;
    let rand_num = 7;

    generate_workout(user_spec_val, rand_num);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive = Cacher::new(|&num| {
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
fn cacher_caches_different_values() {
    let mut c = Cacher::new(|&num| num);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(1, v1);
    assert_eq!(2, v2);
}

#[test]
fn cacher_is_generic() {
    let mut c = Cacher::new(|a: &String| a.len());

    let v1 = c.value(String::from("abc"));
    let v2 = c.value(String::from("efgh"));

    assert_eq!(3, v1);
    assert_eq!(4, v2);
}
