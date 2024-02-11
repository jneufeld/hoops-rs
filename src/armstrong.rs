pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_digits(num);
    let armstrong = get_armstrong_sum(&digits);

    armstrong == num as u128
}

fn get_digits(mut num: u32) -> Vec<u32> {
    let mut result = Vec::new();

    while num > 0 {
        result.push(num % 10);
        num /= 10;
    }

    result
}

fn get_armstrong_sum(digits: &Vec<u32>) -> u128 {
    let mut sum: u128 = 0;
    let power = digits.len() as u32;

    for digit in digits {
        let n = u128::pow(*digit as u128, power);
        sum = sum + n;
    }

    sum
}
