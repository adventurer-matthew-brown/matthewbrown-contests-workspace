//{"name":"A. Square","group":"Codeforces - Codeforces Round 920 (Div. 3)","url":"https://codeforces.com/problemset/problem/1921/A","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 2\n4 5\n1 5\n4 2\n-1 1\n1 -1\n1 1\n-1 -1\n45 11\n45 39\n17 11\n17 39\n","output":"9\n4\n784\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASquare"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut min_x = 10000;
    let mut max_x = -10000;
    let mut min_y = 10000;
    let mut max_y = -10000;

    for _i in 0..4 {
        let x = input.read_int();
        let y = input.read_int();
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    out.print_line((max_x - min_x) * (max_y - min_y));
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
