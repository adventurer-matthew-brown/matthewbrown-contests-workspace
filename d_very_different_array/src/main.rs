//{"name":"D. Very Different Array","group":"Codeforces - Codeforces Round 920 (Div. 3)","url":"https://codeforces.com/problemset/problem/1921/D","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n4 6\n6 1 2 4\n3 5 1 7 2 3\n3 4\n1 1 1\n1 1 1 1\n5 5\n1 2 3 4 5\n1 2 3 4 5\n2 6\n5 8\n8 7 5 8 2 10\n2 2\n4 1\n9 6\n4 6\n8 10 6 4\n3 10 6 1 8 9\n3 5\n6 5 2\n1 7 9 7 2\n5 5\n9 10 6 3 7\n5 9 2 3 9\n1 6\n3\n2 7 10 1 1 5\n","output":"16\n0\n12\n11\n10\n23\n15\n25\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DVeryDifferentArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut a = input.read_long_vec(n);
    let mut b = input.read_long_vec(m);
    a.sort();
    let mut sum_a = vec![0; n + 1];
    for i in 1..=n {
        sum_a[i] = sum_a[i - 1] + a[i - 1];
    }
    b.sort();
    let mut sum_b = vec![0; m + 1];
    for i in 1..=m {
        sum_b[i] = sum_b[i - 1] + b[i - 1];
    }

    let mut ans = 0i64;
    for i in 0..=n {
        // a[0]..a[i], match the largest b's b[m - i]..b[m], total i numbers
        // a[i]..a[n], match the smallest b's, b[0]..b[n - i], total n - i numbers

        if (i == 0 || a[i - 1] <= b[m - i]) && (i == n || a[i] >= b[n - i - 1]) {
            let cur = (sum_b[m] - sum_b[m - i]) - sum_a[i] + (sum_a[n] - sum_a[i]) - sum_b[n - i];
            ans = ans.max(cur);
        }
    }

    out.print_line(ans);
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


/*
1 2 4 6
1 2 3 3 5 7
*/