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

fn search(numbers: &[i32; 10], n: i32) -> usize {
    let mut l = 0;
    let mut r = 9;
    while l < r {
        let m = (l + r) / 2;
        if numbers[m] < n {
            l = m + 1;
        } else if numbers[m] > n {
            r = m - 1;
        }
        else {
            return m;
        }
    }
    return 10;
}

fn main() {
    let n = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("max = {}", maximum(&n));
    println!("find 5's pos =  {}", search(&n, 5));
    for i in n {
        println!("{}'s prime = {}", i, n_prime(i));
    }
    println!("Hello, world!");
}
