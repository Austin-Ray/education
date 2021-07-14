use phf::phf_map;

// Trade binary size for runtime.
static DEST_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "" => "000",
    "M" => "001",
    "D" => "010",
    "DM" => "011",
    "A" => "100",
    "AM" => "101",
    "AD" => "110",
    "ADM"=> "111",
};

static COMP_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "101010",
    "1" => "111111",
    "-1" => "111010",
    "D" => "001100",
    "A" => "110000",
    "M" => "110000",
    "!D" => "001101",
    "!A" => "110001",
    "!M" => "110001",
    "-D" => "001111",
    "-A" => "110011",
    "-M" => "110011",
    "D+1" => "011111",
    "A+1" => "110111",
    "M+1" => "110111",
    "D-1" => "001110",
    "A-1" => "110010",
    "M-1" => "110010",
    "D+A" => "000010",
    "D+M" => "000010",
    "D-A" => "010011",
    "D-M" => "010011",
    "A-D" => "000111",
    "M-D" => "000111",
    "D&A" => "000000",
    "D&M" => "000000",
    "D|A" => "010101",
    "D|M" => "010101",
};

static JUMP_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "" => "000",
    "JGT" => "001",
    "JEQ" => "010",
    "JGE" => "011",
    "JLT" => "100",
    "JNE" => "101",
    "JLE" => "110",
    "JMP" => "111",
};

fn dest(dest: &str) -> String {
    DEST_MAP[dest].to_string()
}

fn comp(comp: &str) -> String {
    COMP_MAP[comp].to_string()
}

fn jump(jump: &str) -> String {
    JUMP_MAP[jump].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected: String,
    }

    impl TestCase {
        fn new(input: &str, expected: &str) -> TestCase {
            TestCase {
                input: input.to_string(),
                expected: expected.to_string(),
            }
        }
    }

    fn test_iter(test_cases: Vec<TestCase>, func: Box<dyn Fn(&str) -> String>) {
        for test_case in test_cases {
            let input = test_case.input;
            let expected = test_case.expected;

            let actual = func(&input);

            assert_eq!(
                actual, expected,
                "Expected {} to be translated into {}, but got {}",
                input, expected, actual
            );
        }
    }

    #[test]
    fn test_dest() {
        let test_cases = vec![
            TestCase::new("", "000"),
            TestCase::new("M", "001"),
            TestCase::new("D", "010"),
            TestCase::new("DM", "011"),
            TestCase::new("A", "100"),
            TestCase::new("AM", "101"),
            TestCase::new("AD", "110"),
            TestCase::new("ADM", "111"),
        ];

        test_iter(test_cases, Box::new(dest));
    }

    #[test]
    fn test_comp() {
        let test_cases = vec![
            TestCase::new("0", "101010"),
            TestCase::new("1", "111111"),
            TestCase::new("-1", "111010"),
            TestCase::new("D", "001100"),
            TestCase::new("A", "110000"),
            TestCase::new("M", "110000"),
            TestCase::new("!D", "001101"),
            TestCase::new("!A", "110001"),
            TestCase::new("!M", "110001"),
            TestCase::new("-D", "001111"),
            TestCase::new("-A", "110011"),
            TestCase::new("-M", "110011"),
            TestCase::new("D+1", "011111"),
            TestCase::new("A+1", "110111"),
            TestCase::new("M+1", "110111"),
            TestCase::new("D-1", "001110"),
            TestCase::new("A-1", "110010"),
            TestCase::new("M-1", "110010"),
            TestCase::new("D+A", "000010"),
            TestCase::new("D+M", "000010"),
            TestCase::new("D-A", "010011"),
            TestCase::new("D-M", "010011"),
            TestCase::new("A-D", "000111"),
            TestCase::new("M-D", "000111"),
            TestCase::new("D&A", "000000"),
            TestCase::new("D&M", "000000"),
            TestCase::new("D|A", "010101"),
            TestCase::new("D|M", "010101"),
        ];

        test_iter(test_cases, Box::new(comp));
    }

    #[test]
    fn test_jump() {
        let test_cases = vec![
            TestCase::new("", "000"),
            TestCase::new("JGT", "001"),
            TestCase::new("JEQ", "010"),
            TestCase::new("JGE", "011"),
            TestCase::new("JLT", "100"),
            TestCase::new("JNE", "101"),
            TestCase::new("JLE", "110"),
            TestCase::new("JMP", "111"),
        ];

        test_iter(test_cases, Box::new(jump));
    }
}
