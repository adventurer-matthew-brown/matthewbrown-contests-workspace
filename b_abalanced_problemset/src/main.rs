//{"name":"B. A Balanced Problemset?","group":"Codeforces - Codeforces Round 921 (Div. 2)","url":"https://codeforces.com/problemset/problem/1925/B","interactive":false,"timeLimit":1500,"tests":[{"input":"3\n10 3\n5 5\n420 69\n","output":"2\n1\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BABalancedProblemset"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = Vec<usize>;

fn find_all_divisors(depth: usize, cur: usize, px: &Vec<usize>, kx: &Vec<usize>, divisors: &mut Vec<usize>) {
    if depth == px.len() {
        divisors.push(cur);
        return;
    }
    let mut mul = 1;
    for i in 0..=kx[depth] {
        find_all_divisors(depth + 1, cur * mul, px, kx, divisors);
        mul *= px[depth];
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, primes: &PreCalc) {
    let mut x = input.read_size();
    let mut ans = x;
    let n = input.read_size();
    let mut px = vec![0; 0];
    let mut kx = vec![0; 0];

    for &prime in primes {
        let mut cur_k = 0;
        while x % prime == 0 {
            x /= prime;
            cur_k += 1;
        }
        if cur_k > 0 {
            px.push(prime);
            kx.push(cur_k);
        }
        if x < prime {
            break;
        }
    }

    if x > 1 {
        px.push(x);
        kx.push(1);
    }

    if px.len() == 1 && kx[0] == 1 {
        ans = if n == 1 { ans } else { 1 };
    } else {
        let mut divisors = vec![0usize; 0];
        find_all_divisors(0, 1, &px, &kx, &mut divisors);
        divisors.sort();
        for d in divisors {
            if d >= n {
                ans = ans / d;
                break;
            }
        }
    }

    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    const MAX_PRIME: usize = 10000;
    let mut pre_calc = vec![0; 0];
    let mut not_prime = [false; MAX_PRIME + 1];

    for i in 2..(MAX_PRIME + 1) {
        if not_prime[i] {
            continue;
        }
        pre_calc.push(i);
        let mut k = i * i;
        while k <= MAX_PRIME {
            not_prime[k] = true;
            k += i;
        }
    }

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}


//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
