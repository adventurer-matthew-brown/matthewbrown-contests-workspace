//{"name":"E. Increasing Subsequences","group":"Codeforces - Educational Codeforces Round 161 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1922/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n5\n13\n37\n","output":"1\n0\n3\n0 1 0\n5\n2 2 3 4 2\n7\n-1 -1 0 0 2 3 -1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EIncreasingSubsequences"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

/**
 * 0 1 2 3 4 .. N-1
 * 
 * 2^N
 */
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let x = input.read_long();
    let mut bits = vec![0; 0];
    for i in 0..63 {
        if ((1 << i) & x) > 0 {
            bits.push(i);
        }
    }

    let mut ans = vec![0; 0];
    let max_bit = *bits.last().unwrap();
    let mut ans: Vec<i32> = (0..max_bit).collect();
    bits.pop();
    let mut counter = -(bits.len() as i32);
    for bit_pos in bits {
        let insert_pos = max_bit - bit_pos;
        ans.insert(insert_pos as usize, counter);
        counter += 1;
    }

    out.print_line(ans.len());
    for ans_elem in ans {
        out.print(ans_elem);
        out.print(" ");
    }
    out.print_line("");
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
