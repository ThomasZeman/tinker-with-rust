// Thoughts:
// - After removing any k digits, all strings will have the same length (excluding special case of leading zeros)
// - Start scanning from higher to lower digits
//
// Example:
// 1432219, k=3 -> 1219
// 1452219, k=3 -> 1219
// *  * **
// 1462289, k=3 -> 1228
// *  *** 
// 1426354, k=3 -> 1234
// * * * *
// 1426354, k=4 -> 123
// * * * 
// 8721354, k=3 -> 1354
//    ****
// 1426354, k=1 -> 126354
// * *****
// 10200,   k=1 -> 200
//  **** 
// 10,      k=2 -> 0
//
// Add digit by digit until digit is found which is smaller than n previous digits. Then remove previous ->
// Add digit by digit, if current digit is smaller then previous, remove previous until current is no longer smaller than previous

pub fn remove_kdigits_impl(num: Vec<u32>, mut k: i32) -> Option<Vec<u32>> {
    // According to description, if all digits can be removed return 0
    if num.len() == k as usize {
        return Some(vec![0]);
    }
    let mut result = Vec::new();
    for current_digit in num {
        // While previous digit is bigger than current digit remove previous digit
        while result.len() != 0 && result[result.len() - 1] > current_digit && k != 0 {
            result.pop();
            println!("< {} {:?}", current_digit, result);
            k-=1;
        }
        // Don't add leading zeros
        if current_digit != 0 || result.len() != 0 {
            result.push(current_digit);
            println!("> {} {:?}", current_digit, result);
        } else {
            println!("i 0");
        }
    }
    // If k is not zero, trailing digits need to be removed
    while k > 0 {
        k-=1;
        result.pop();
    }
    return Some(result);
}

pub fn remove_kdigits(num: String, k: i32) -> String {
    return remove_kdigits_impl(
        num.chars().map(|c| c.to_digit(10).unwrap()).collect(), k)
    .unwrap().iter().map(|d| d.to_string()).collect();
}

fn main() {
    for number in vec![("1432219", 3), ("1452219", 3), ("1462289", 3), ("1426354", 3), ("1426354", 4), ("8721354", 3), ("1426354", 1), ("10200", 1), ("10", 2)].iter() {
        println!("{}:{} -> {}", number.0, number.1, remove_kdigits(number.0.to_string(), number.1)); 
    }
}
