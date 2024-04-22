#[derive(Debug, PartialEq, Eq)]
// an enum of different numbers
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    // the number should not be zero
    if num == 0 {
        return None;
    }
// range of factors should not include the number
    let aliquot_sum = (1..num)
    // checking where every item in the range can divide the number and leaves on reminder
        .filter(|&i| num % i == 0)
        .sum::<u64>();

    match aliquot_sum {
        sum if sum == num => Some(Classification::Perfect),
        sum if sum > num => Some(Classification::Abundant),
        _ => Some(Classification::Deficient),
    }
}


