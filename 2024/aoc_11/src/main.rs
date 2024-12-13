use std::io::Read;

type StoneValT = u32;
type Stones = Vec<StoneValT>;

fn get_input_string() -> String {
    let mut input = String::default();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Unable to read stdin");
    input
}

fn parse_input(input: &str) -> Stones {
    let words = input.split_whitespace();
    let mut stones = Stones::default();
    for word in words {
        let val = StoneValT::from_str_radix(word, 10).expect("should be a number");
        stones.push(val);
    }
    stones
}

fn blink(stones: Stones) -> Stones {
    stones
        .into_iter()
        .flat_map(|val| {
            if val == 0 {
                return [1].to_vec();
            }
            {
                let s = val.to_string();
                let len = s.len();
                if len % 2 == 0 {
                    let new_len = len / 2;
                    let (s1, s2) = s.split_at(new_len);
                    return [
                        StoneValT::from_str_radix(s1, 10).unwrap(),
                        StoneValT::from_str_radix(s2, 10).unwrap(),
                    ]
                    .to_vec();
                }
            }

            [val * 2024].to_vec()
        })
        .collect()
}

fn main() {
    let input = get_input_string();
    let mut stones = parse_input(&input);
    stones = blink(stones);
    println!("{:?}", stones);
}
