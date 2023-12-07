use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::iter::zip;
use std::process;

#[derive(Debug)]
struct Hand {
    card: String,
    bid: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn main() {
    let digits = read_input(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let total = run(&digits).unwrap_or_else(|err| {
        println!("Calculation error: {err}");
        process::exit(1);
    });

    println!("total = {:#?}", total);
}

fn read_input(mut args: impl Iterator<Item = String>) -> Result<String, &'static str> {
    args.next();

    let file_path = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file path"),
    };

    Ok(file_path)
}

fn run(file_path: &str) -> Result<(u64, u64), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let part_1 = calc_part_1(&contents);

    let part_2 = calc_part_2(&contents);

    Ok((part_1, part_2))
}

fn calc_part_1(contents: &str) -> u64 {
    let cards = HashMap::from([
        ('2', 0),
        ('3', 1),
        ('4', 2),
        ('5', 3),
        ('6', 4),
        ('7', 5),
        ('8', 6),
        ('9', 7),
        ('T', 8),
        ('J', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);
    calc(&contents, compare_hands, match_hand_part_1, &cards)
}

fn match_hand_part_1(hand: &Hand) -> Kind {
    let card_count = count_card(hand);

    match card_count.len() {
        5 => Kind::HighCard,
        4 => Kind::OnePair,
        3 => {
            let three_of_a_kind = card_count.iter().filter(|entry| *entry.1 == 3).count();
            if three_of_a_kind == 1 {
                return Kind::ThreeOfAKind;
            }
            Kind::TwoPair
        }
        2 => {
            let four_of_a_kind = card_count.iter().filter(|entry| *entry.1 == 4).count();
            if four_of_a_kind == 1 {
                return Kind::FourOfAKind;
            }
            Kind::FullHouse
        }
        _ => Kind::FiveOfAKind,
    }
}

fn calc_part_2(contents: &str) -> u64 {
    let cards = HashMap::from([
        ('J', 0),
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);
    calc(&contents, compare_hands,  match_hand_part_2, &cards)
}

fn calc(
    contents: &str,
    compare_func: fn(
        first: &Hand,
        second: &Hand,
        cards: &HashMap<char, u64>,
        match_hand_func: fn(hand: &Hand) -> Kind,
    ) -> Ordering,
    match_hand_func: fn(hand: &Hand) -> Kind,
    cards: &HashMap<char, u64>,
) -> u64 {
    let mut hands = parse_input(&contents);
    hands.sort_by(|first, second| compare_func(first, second, cards, match_hand_func));

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank as u64 + 1))
        .sum()
}

fn compare_hands(
    first: &Hand,
    second: &Hand,
    cards: &HashMap<char, u64>,
    match_hand_func: fn(hand: &Hand) -> Kind,
) -> Ordering {
    let first_hand = match_hand_func(first);
    let second_hand = match_hand_func(second);

    if first_hand == second_hand {
        for item in zip(first.card.chars(), second.card.chars()) {
            if item.0 != item.1 {
                if cards[&item.0] < cards[&item.1] {
                    return Ordering::Less;
                } else if cards[&item.0] > cards[&item.1] {
                    return Ordering::Greater;
                }
                // assume no equal here
            }
        }
    }
    first_hand.cmp(&second_hand)
}

fn match_hand_part_2(hand: &Hand) -> Kind {
    let card_count = count_card(hand);

    match card_count.len() {
        5 => {
            if card_count.get(&'J').is_some() {
                return Kind::OnePair;
            }
            Kind::HighCard
        }
        4 => {
            if card_count.get(&'J').is_some() {
                return Kind::ThreeOfAKind;
            }
            Kind::OnePair
        }
        3 => {
            let three_of_a_kind = card_count.iter().filter(|entry| *entry.1 == 3).count();
            if three_of_a_kind == 1 {
                if card_count.get(&'J').is_some() {
                    return Kind::FourOfAKind;
                }
                return Kind::ThreeOfAKind;
            } else {
                if let Some(num_of_j) = card_count.get(&'J') {
                    return match num_of_j {
                        2 => Kind::FourOfAKind,
                        _ => Kind::FullHouse,
                    };
                }
                Kind::TwoPair
            }
        }
        2 => {
            if card_count.get(&'J').is_some() {
                return Kind::FiveOfAKind;
            }
            let four_of_a_kind = card_count.iter().filter(|entry| *entry.1 == 4).count();
            if four_of_a_kind == 1 {
                return Kind::FourOfAKind;
            }
            Kind::FullHouse
        }
        _ => Kind::FiveOfAKind,
    }
}

fn count_card(hand: &Hand) -> HashMap<char, u32> {
    hand.card.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}

fn parse_input(contents: &str) -> Vec<Hand> {
    contents
        .lines()
        .map(|line| {
            let mut iter = line.split(" ");
            let card = iter.next().unwrap().to_owned();
            let bid = iter.next().unwrap().parse::<u64>().unwrap();
            Hand { card, bid }
        })
        .collect::<Vec<Hand>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_of_a_kind() {
        let hand = Hand {
            card: String::from("AAAAA"),
            bid: 123,
        };
        assert_eq!(match_hand_part_1(&hand), Kind::FiveOfAKind);
    }

    #[test]
    fn four_of_a_kind() {
        let hand = Hand {
            card: String::from("TAAAA"),
            bid: 123,
        };
        assert_eq!(match_hand_part_1(&hand), Kind::FourOfAKind);
    }

    #[test]
    fn three_of_a_kind() {
        let hand = Hand {
            card: String::from("ATAKA"),
            bid: 123,
        };
        assert_eq!(match_hand_part_1(&hand), Kind::ThreeOfAKind);
    }

    #[test]
    fn full_house() {
        let hand = Hand {
            card: String::from("ATATA"),
            bid: 123,
        };
        assert_eq!(match_hand_part_1(&hand), Kind::FullHouse);
    }

    #[test]
    fn two_pairs() {
        let hand = Hand {
            card: String::from("ATATK"),
            bid: 123,
        };
        assert_eq!(match_hand_part_1(&hand), Kind::TwoPair);
    }

    #[test]
    fn one_pair() {
        let hand = Hand {
            card: String::from("3T2TK"),
            bid: 123,
        };
        assert_eq!(match_hand_part_1(&hand), Kind::OnePair);
    }

    #[test]
    fn high_card() {
        let hand = Hand {
            card: String::from("23456"),
            bid: 123,
        };
        assert_eq!(match_hand_part_1(&hand), Kind::HighCard);
    }

    #[test]
    fn high_card_elevation_with_joker() {
        let hand = Hand {
            card: String::from("2345J"),
            bid: 123,
        };
        assert_eq!(match_hand_part_2(&hand), Kind::OnePair);
    }

    #[test]
    fn high_card_no_elevation() {
        let hand = Hand {
            card: String::from("23456"),
            bid: 123,
        };
        assert_eq!(match_hand_part_2(&hand), Kind::HighCard);
    }

    #[test]
    fn one_pair_elevation_with_joker() {
        let hand = Hand {
            card: String::from("2324J"),
            bid: 123,
        };
        assert_eq!(match_hand_part_2(&hand), Kind::ThreeOfAKind);
    }

    #[test]
    fn one_pair_no_elevation() {
        let hand = Hand {
            card: String::from("23245"),
            bid: 123,
        };
        assert_eq!(match_hand_part_2(&hand), Kind::OnePair);
    }

    #[test]
    fn three_of_a_kind_elevation_with_one_joker() {
        let hand = Hand {
            card: String::from("2223J"),
            bid: 123,
        };
        assert_eq!(match_hand_part_2(&hand), Kind::FourOfAKind);
    }

    #[test]
    fn three_of_a_kind_no_elevation() {
        let hand = Hand {
            card: String::from("22234"),
            bid: 123,
        };
        assert_eq!(match_hand_part_2(&hand), Kind::ThreeOfAKind);
    }

    #[test]
    fn two_pair_elevation_with_two_joker() {
        let hand = Hand {
            card: String::from("22JJ4"),
            bid: 123,
        };
        assert_eq!(match_hand_part_2(&hand), Kind::FourOfAKind);
    }

    #[test]
    fn two_pair_elevation_with_one_joker() {
        let hand = Hand {
            card: String::from("2233J"),
            bid: 123,
        };
        assert_eq!(match_hand_part_2(&hand), Kind::FullHouse);
    }

    #[test]
    fn two_pair_no_elevation() {
        let hand = Hand {
            card: String::from("22334"),
            bid: 123,
        };
        assert_eq!(match_hand_part_2(&hand), Kind::TwoPair);
    }

    #[test]
    fn full_house_no_elevation() {
        let hand = Hand {
            card: String::from("22333"),
            bid: 123,
        };
        assert_eq!(match_hand_part_2(&hand), Kind::FullHouse);
    }

    #[test]
    fn full_house_elevation_with_joker() {
        let hand = Hand {
            card: String::from("JJ333"),
            bid: 123,
        };
        assert_eq!(match_hand_part_2(&hand), Kind::FiveOfAKind);
    }

    #[test]
    fn four_of_a_kind_elevation_with_joker() {
        let hand = Hand {
            card: String::from("3333J"),
            bid: 123,
        };
        assert_eq!(match_hand_part_2(&hand), Kind::FiveOfAKind);
    }

    #[test]
    fn four_of_a_kind_no_elevation() {
        let hand = Hand {
            card: String::from("33334"),
            bid: 123,
        };
        assert_eq!(match_hand_part_2(&hand), Kind::FourOfAKind);
    }
}
