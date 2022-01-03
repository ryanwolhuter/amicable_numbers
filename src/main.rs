fn main() {
    /*
    Amicable numbers are two different natural numbers
    related in such a way that the sum of the proper divisors
    of each is equal to the other number.
    That is, σ(a)=b and σ(b)=a,
    where σ(n) is equal to the sum of positive divisors of n
    */
    let sum_of_amicable_pairs = sum_amicable_pairs(10_000);
    println!("{}", sum_of_amicable_pairs);
}

fn sum_of_divisors(n: u32) -> u32 {
    let range = 1..n / 2 + 1;
    let divisors = range.filter(|x| n % x == 0);
    divisors.sum()
}

fn sum_amicable_pairs(limit: u32) -> u32 {
    let range = 1..limit;
    // build pairs of tuples where one is the sum of divisors of the other
    let pairs = range
        .map(|n| (n, sum_of_divisors(n)))
        .filter(|&(n, sum_of_divisors)| n > sum_of_divisors);

    let mut sum_of_amicable_pairs = 0;

    for (x, y) in pairs {
        // check if the sum of divisors of the other is equal to the first
        if sum_of_divisors(y) == x {
            // if so, add the two numbers to the sum
            sum_of_amicable_pairs += x + y;
        }
    }

    sum_of_amicable_pairs
}
