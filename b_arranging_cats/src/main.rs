//{"name":"B. Arranging Cats","group":"Codeforces - Codeforces Round 920 (Div. 3)","url":"https://codeforces.com/problemset/problem/1921/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n5\n10010\n00001\n1\n1\n1\n3\n000\n111\n4\n0101\n1010\n3\n100\n101\n8\n10011001\n11111110\n","output":"2\n0\n3\n2\n1\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BArrangingCats"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut a = vec!['0'; n];
    let mut b = vec!['0'; n];
    for i in 0..n {
        a[i] = input.read_char();
    }

    let mut need_set = 0;
    let mut need_clear = 0;
    for i in 0..n {
        b[i] = input.read_char();
        if a[i] == '0' && b[i] == '1' {
            need_set += 1;
        }
        if a[i] == '1' && b[i] == '0' {
            need_clear += 1;
        }
    }

    out.print_line(need_set + need_clear - need_set.min(need_clear));
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
