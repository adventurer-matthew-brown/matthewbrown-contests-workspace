//{"name":"A. Tricky Template","group":"Codeforces - Educational Codeforces Round 161 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1922/A","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\na\nb\nc\n2\naa\nbb\naa\n10\nmathforces\nluckforces\nadhoccoder\n3\nacc\nabd\nabc\n","output":"YES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATrickyTemplate"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut a = vec!['\0'; n];
    let mut b = vec!['\0'; n];
    let mut c = vec!['\0'; n];
    for i in 0..n { a[i] = input.read_char(); }
    for i in 0..n { b[i] = input.read_char(); }
    for i in 0..n { c[i] = input.read_char(); }

    let mut ans = false;

    for i in 0..n {
        if a[i] == b[i] && c[i] != a[i] {
            ans = true;
            break;
        }
        if a[i] != b[i] && c[i] != a[i] && c[i] != b[i] {
            ans = true;
            break;
        }
    }

    out.print_line(if ans { "YES" } else { "NO" });
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
