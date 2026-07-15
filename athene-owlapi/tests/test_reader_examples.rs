use athene_owlapi::reader::parse_reader;
use std::{fs::File, io::BufReader};

macro_rules! make_test_case {
    ($test_name:ident, $file_name:literal) => {
        #[test]
        fn $test_name() {
            let file = File::open(format!("tests/examples/{}.owl", $file_name))
                .expect("Could not open test case example file");
            let reader = BufReader::new(file);
            let result = parse_reader(reader, false);

            println!("{result:#?}");

            assert!(result.is_ok());
        }
    };
}

make_test_case!(test_nomagic_example_001, "nomagic-example-001");
make_test_case!(test_nomagic_example_002, "nomagic-example-002");
make_test_case!(test_nomagic_example_003, "nomagic-example-003");
make_test_case!(test_nomagic_example_004, "nomagic-example-004");
