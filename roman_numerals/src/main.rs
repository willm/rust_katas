use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn get_arabic<'a>(
    numerals: &'a HashMap<char, i32>,
    current: char,
    previous: Option<char>,
) -> &i32 {
    match previous {
        Some(p) => {
            for special_case in [
                ('I', 'V', 3),
                ('I', 'X', 8),
                ('X', 'L', 30),
                ('C', 'M', 800),
            ].iter() {
                if p == special_case.0 && current == special_case.1 {
                    return &special_case.2;
                }
            }
        }
        None => {}
    };
    numerals
        .get(&current)
        .expect("Unknown numeral found")
}

fn roman(numeral: &str) -> i32 {
    let mut numerals = HashMap::new();
    numerals.insert('I', 1);
    numerals.insert('V', 5);
    numerals.insert('X', 10);
    numerals.insert('L', 50);
    numerals.insert('C', 100);
    numerals.insert('M', 1000);

    let mut arab = 0;
    for (pos, c) in numeral.chars().enumerate() {
        arab += get_arabic(
            &numerals,
            c,
            if pos == 0 {
                None
            } else {
                (numeral.chars().nth(pos - 1))
            },
        );
    }
    arab
}

#[test]
fn conversion_tests() {
    for case in [
        ("", 0),
        ("I", 1),
        ("IV", 4),
        ("V", 5),
        ("VI", 6),
        ("IX", 9),
        ("X", 10),
        ("XI", 11),
        ("XIV", 14),
        ("XV", 15),
        ("XX", 20),
        ("XL", 40),
        ("L", 50),
        ("C", 100),
        ("CML", 950),
        ("M", 1000),
        ("MCMXII", 1912),
        ("MMXX", 2020),
    ]
    .iter()
    {
        println!("{:?}", case.0);
        assert_eq!(roman(case.0), case.1);
    }
}
