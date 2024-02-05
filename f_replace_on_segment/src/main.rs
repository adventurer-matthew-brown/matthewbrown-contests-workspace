//{"name":"F. Replace on Segment","group":"Codeforces - Educational Codeforces Round 161 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1922/F","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n3 2\n1 2 1\n6 3\n1 2 3 1 2 3\n12 3\n3 1 3 1 2 1 1 2 3 1 1 3\n","output":"1\n2\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FReplaceOnSegment"}}}

use std::cmp::min;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let x = input.read_size();
    let a = input.read_size_vec(n);

    let mut same = vec![vec![vec![false; x + 1]; n + 1]; n + 1];
    let mut has = vec![vec![vec![false; x + 1]; n + 1]; n + 1];
    let mut g = vec![vec![vec![1e9 as i32; x + 1]; n + 1]; n + 1];
    let mut f =  vec![vec![vec![0; x + 1]; n + 1]; n + 1];

    for len in 0..n {
        for l in 0..(n - len) {
            let r = l + len;
            let mut min_f = 1e9 as i32;
            let mut second_min_f = 1e9 as i32;
            for k in 1..=x {
                same[l][r][k] = a[l] == k && (l == r || same[l + 1][r][k]);
                has[l][r][k] = a[l] == k || (l < r && has[l + 1][r][k]);

                g[l][r][k] = if !has[l][r][k] { 0 } else { 
                    let mut result = 1e9 as i32;
                    for split in l..r {
                        result = result.min(g[l][split][k] + g[split + 1][r][k]);
                    }
                    result
                };

                f[l][r][k] = if same[l][r][k] { 0 } else {
                    let mut result_f = 1e9 as i32;
                    let result_g = g[l][r][k] + 1;
                    for split in l..r {
                        result_f = result_f.min(f[l][split][k] + f[split + 1][r][k]);
                    }
                    min(result_f, result_g)
                };

                if f[l][r][k] <= min_f {
                    second_min_f = min_f;
                    min_f = f[l][r][k];
                } else if f[l][r][k] < second_min_f {
                    second_min_f = f[l][r][k];
                }
            }

            // g won't matter if x == 1 
            for k in 1..=x {
                g[l][r][k] = g[l][r][k].min(if f[l][r][k] == min_f { second_min_f } else { min_f })
            }
        }
    }

    if x == 1 {
        out.print_line(0);
    } else {
        let mut ans = 1e9 as i32;
        for k in 1..=x {
            ans = ans.min(f[0][n - 1][k]);
        }
        out.print_line(ans);
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
