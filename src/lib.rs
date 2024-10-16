use std::collections::HashMap;
// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(str: &str) {
    alert(&format!("Hello, {}!", str))
}

#[wasm_bindgen]
pub fn fibonacci(num: usize) -> usize {
    let mut cache: HashMap<usize, usize> = HashMap::new();
    let result = fibonacci_impl(&num, &mut cache);
    result
}

fn fibonacci_impl(num: &usize, cache: &mut HashMap<usize, usize>) -> usize {
    match cache.get(num) {
        Some(&value) => value,
        None => {
            let result = match num {
                0 => 0,
                1 => 1,
                _ => fibonacci_impl(&(num - 1), cache) + fibonacci_impl(&(num - 2), cache),
            };
            cache.insert(*num, result);
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fibonacci(10);
        assert_eq!(result, 55);
    }
}