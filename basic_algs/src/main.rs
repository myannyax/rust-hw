fn maximum(numbers: &[i32; 10]) -> i32 {
    let mut max = numbers[0];
    for i in numbers {
        if max < *i {
            max = *i;
        }
    }
    max
}

fn n_prime(n: i32) -> i32 {
    let mut res = 1;
    let mut kek = 1;
    loop {
        let mut is_prime = true;
        for i in 1..res {
            if !(i == 1 || (res % i) != 0) {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            if kek == n {
                return res;
            } else {
                kek += 1;
            }
        }
        res += 1
    }
}

fn search(numbers: &[i32; 10], n: i32) -> Option<usize> {
    let mut l = 0;
    let mut r = 10;
    while l < r {
        let m = (l + r) / 2;
        if numbers[m] < n {
            l = m + 1;
        } else if numbers[m] > n {
            r = m;
        }
        else {
            return Option::Some(m);
        }
    }
    return Option::None;
}

fn main() {
    let n = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("max = {}", maximum(&n));
    println!("find 5's pos =  {}", search(&n, 5).unwrap_or(999));
    for i in n {
        println!("{}'s prime = {}", i, n_prime(i));
    }
    for i in &n {
        println!("{}'s index = {}", i, search(&n, *i).unwrap_or(999));
    };
    let n = [0, 2, 4, 5, 8, 9, 10, 11, 13, 18];
    for i in &n {
        println!("{}'s index = {}", i, search(&n, *i).unwrap_or(999));
    };
}
