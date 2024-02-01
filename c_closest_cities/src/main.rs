//{"name":"C. Closest Cities","group":"Codeforces - Educational Codeforces Round 161 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1922/C","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n5\n0 8 12 15 20\n5\n1 4\n1 5\n3 4\n3 2\n5 1\n","output":"3\n8\n1\n4\n14\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CClosestCities"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut a = vec![0; n + 1];
    let mut nearest = vec![0; n + 1];
    for i in 1..=n {
        a[i] = input.read_int();
    }
    nearest[1] = 2;
    nearest[n] = n - 1;

    for i in 2..n {
        let dl = (a[i] - a[i - 1]).abs();
        let dr = (a[i] - a[i + 1]).abs();
        nearest[i] = if dl < dr { i - 1 } else { i + 1 };
    }

    let mut d_lr = vec![0; n + 1];
    d_lr[n] = 0;
    for i in (1..n).rev() {
        d_lr[i] = d_lr[i + 1] + (a[i + 1] - a[i]).abs();
        if nearest[i] == i + 1 {
            d_lr[i] = d_lr[i + 1] + 1;
        }
    }

    let mut d_rl = vec![0; n + 1];
    d_rl[1] = 0;
    for i in 2..=n {
        d_rl[i] = d_rl[i - 1] + (a[i] - a[i - 1]).abs();
        if nearest[i] == i - 1 {
            d_rl[i] = d_rl[i - 1] + 1;
        }
    }

    let m = input.read_size();
    for _i in 0..m {
        let x = input.read_size();
        let y = input.read_size();
        if x < y {
            out.print_line(d_lr[x] - d_lr[y]);
        } else {
            out.print_line(d_rl[x] - d_rl[y]);
        }
    }
    
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

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
