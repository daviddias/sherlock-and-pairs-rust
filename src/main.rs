use std::io;
use std::collections::{HashMap};

fn main() {
    let mut reader = io::stdin();

    let n_experiments: int = from_str(reader.read_line().unwrap().as_slice().trim()).unwrap();
    // println!("n_experiments {}", n_experiments);
    // explanation
    // 1. reader.read_line()      - read the line                                 
    // 2. .unwrap()               - unwrap (Rust creats this struct where it returns the result or error in one package) 
    // 3. .as_slice()             - Conver to &str - http://stackoverflow.com/questions/24158114/rust-string-versus-str/24159933#24159933
    // 4. .trim                   - that you expect
    // 5. from_str(...).unwrap(); - Convert to int

    for _ in range(0, n_experiments) {
        let n_numbers: int = from_str(reader.read_line().unwrap().as_slice().trim()).unwrap();
        
        // http://stackoverflow.com/questions/26536871/convert-a-string-of-numbers-to-an-array-vector-of-ints-in-rust/26537398#26537398
        let numbers: Vec<int> = 
            reader.read_line().unwrap().as_slice()  // (1)
                  .split(' ').map(|s| s.trim())     // (2)
                  .filter(|s| !s.is_empty())        // (3)
                  .map(|s| from_str(s).unwrap())    // (4)
                  .collect();                       // (5)    
        let counts: Vec<int> = vec_count_elem(numbers);

        let mut total: int = 0i;
        for count in counts.iter() {
            if *count > 1i {
                total = total + smart_permutations(*count);
            }
        }
        println!("{}", total);
    }
}


fn smart_permutations(n: int) -> int { n * (n-1) }

fn vec_count_elem(vec: Vec<int>) -> Vec<int> {
    let mut counts: HashMap<int, int> = HashMap::new();;
    let mut result: Vec<int> = Vec::new();

    for el in vec.iter() {
        // http://stackoverflow.com/questions/26546215/using-rust-hashmaps-find-is-not-returning-the-expected-value-for-match/26546418?noredirect=1#comment41716396_26546418
        let found = match counts.find_mut(el) {
            Some(count) => {
                *count = *count + 1;
                true
            }
            None => false // inserting in case of Node does not work currently
        };
        if !found {
            counts.insert(*el, 1);
        }
    }
    for (key, val) in counts.iter() {
        result.push(*val);
    }

    return result;   
}