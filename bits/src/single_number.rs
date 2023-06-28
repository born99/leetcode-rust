use std::collections::HashMap;

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut hash: HashMap<i32, i16> = HashMap::default();
    for element in nums {
        if hash.contains_key(&element) {
            hash.insert(element, 2);
            continue;
        }
        hash.insert(element, 1);
    }

    for (k,v) in hash {
        
        if v == 1 {
            return k;
        }
    } 

    0
}


pub fn single_number_v2(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |ans, element| ans^element)
}

pub fn single_number_v3(nums: Vec<i32>) -> i32 {
    let mut result = 0;

    for element in nums {
        result = result ^ element
    }

    result
}