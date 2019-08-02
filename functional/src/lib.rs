use std::thread;
use std::time::Duration;

pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simulate() {
        assert_eq!(10, simulated_expensive_calculation(10));
    }

    #[test]
    fn test_cache() {
        let mut cache = Cache::new(|num|num);
        cache.value(1);
        let value2 = cache.value(1);

        assert_eq!(value2, 1);
    }

    #[test]
    fn iterator_sum(){
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total = v1_iter.sum();
        assert_eq!(6, total);
    }
}

pub fn generate_workout(indensity: u32, random_number: u32) {
    let mut expensive_result = Cache::new(|num| {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if indensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(indensity));
        println!("Next, do {} situps!", expensive_result.value(indensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(indensity));
        }
    }
}

struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cache<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cache<T>{
        Cache{
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value{
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}