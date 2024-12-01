use std::cmp::{min, Ordering};

struct Hand<'a> {
    cards: &'a str,
    bid: u32,
    hand_type: HandType,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

pub fn calculate1(input: &str) -> String {
    let mut hands: Vec<Hand> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let cards = &line[0..5];
            let bid = (&line[6..]).parse().unwrap();
            let hand_type = get_hand_type(cards);
            Hand { cards, bid, hand_type }
        }).collect();

    hands.sort_by(|a, b| {
        let ordering = a.hand_type.cmp(&b.hand_type);
        if ordering != Ordering::Equal {
            return ordering;
        }

        for i in 0..5 {
            let o = cmp_cards(a.cards.chars().nth(i).unwrap(), b.cards.chars().nth(i).unwrap());
            if o != Ordering::Equal {
                return o;
            }
        }

        return Ordering::Equal;
    });

    let sum = hands.iter()
        .enumerate()
        .fold(0, |acc: u32, (i, hand)| acc + hand.bid * (i as u32 + 1));

    return sum.to_string();
}

fn cmp_cards(a: char, b: char) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }
    let va = card_value(a);
    let vb = card_value(b);

    return va.cmp(&vb);
}

fn cmp_cards_joker(a: char, b: char) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }
    let va = card_value_joker(a);
    let vb = card_value_joker(b);

    return va.cmp(&vb);
}

fn card_value(card: char) -> u8 {
    if card == 'A' {
        return 14;
    }
    if card == 'K' {
        return 13;
    }
    if card == 'Q' {
        return 12;
    }
    if card == 'J' {
        return 11;
    }
    if card == 'T' {
        return 10;
    }
    return card as u8 - '0' as u8;
}

fn card_value_joker(card: char) -> u8 {
    if card == 'A' {
        return 14;
    }
    if card == 'K' {
        return 13;
    }
    if card == 'Q' {
        return 12;
    }
    if card == 'J' {
        return 1;
    }
    if card == 'T' {
        return 10;
    }
    return card as u8 - '0' as u8;
}

fn get_hand_type(cards: &str) -> HandType {
    let mut counts: [u8; 5] = [0; 5];
    for i in 0..5 {
        let card = cards.chars().nth(i).unwrap();
        let mut count = 1;
        for j in 0..5 {
            if i == j {
                continue;
            }

            let card_b = cards.chars().nth(j).unwrap();
            if card == card_b {
                count += 1
            }
        }
        counts[i] = count;
    }

    let sum: u8 = counts.iter().sum();

    if sum == 5 {
        return HandType::HighCard;
    }
    if sum == 7 {
        return HandType::OnePair;
    }
    if sum == 9 {
        return HandType::TwoPair;
    }
    if sum == 11 {
        return HandType::ThreeOfKind;
    }
    if sum == 13 {
        return HandType::FullHouse;
    }
    if sum == 17 {
        return HandType::FourOfKind;
    }
    if sum == 25 {
        return HandType::FiveOfKind;
    }

    panic!()
}

fn get_hand_type_joker(cards: &str) -> HandType {
    let mut counts: [u8; 5] = [0; 5];
    for i in 0..5 {
        let card = cards.chars().nth(i).unwrap();
        let mut count = 1;
        for j in 0..5 {
            if i == j {
                continue;
            }

            let card_b = cards.chars().nth(j).unwrap();
            if card == card_b {
                count += 1
            }
        }
        counts[i] = count;
    }

    let number_of_j: u8 = cards.chars().filter(|x| *x == 'J').count() as u8;
    if number_of_j > 0 {
        let max: u8 = cards.chars().enumerate()
            .filter(|(_i, x)| *x != 'J')
            .map(|(i, _x)| counts[i])
            .max()
            .or(Some(0))
            .unwrap();

        let mut j_left: u8 = number_of_j;
        let mut card_max = ' ';
        for i in 0..5 {
            let card = cards.chars().nth(i).unwrap();
            if counts[i] == max || card == 'J' {
                if card_max == ' ' && card != 'J' {
                    card_max = card;
                }
                if card == 'J' {
                    counts[i] = min(5, max + number_of_j)
                } else if j_left > 0 {
                    j_left -= 1;
                }
                if card == card_max {
                    counts[i] += number_of_j;
                }
            }
        }
    }

    let sum: u8 = counts.iter().sum();

    if sum == 5 {
        return HandType::HighCard;
    }
    if sum == 7 {
        return HandType::OnePair;
    }
    if sum == 9 {
        return HandType::TwoPair;
    }
    if sum == 11 {
        return HandType::ThreeOfKind;
    }
    if sum == 13 {
        return HandType::FullHouse;
    }
    if sum == 17 {
        return HandType::FourOfKind;
    }
    if sum == 25 {
        return HandType::FiveOfKind;
    }

    panic!()
}


pub fn calculate2(input: &str) -> String {
    let mut hands: Vec<Hand> = input.lines()
        .map(|line| line.trim_start_matches(|c: char| c.is_whitespace()))
        .filter(|it| it.len() != 0)
        // .inspect(|line| { dbg!(line); })
        .map(|line| {
            let cards = &line[0..5];
            let bid = (&line[6..]).parse().unwrap();
            let hand_type = get_hand_type_joker(cards);
            Hand { cards, bid, hand_type }
        }).collect();

    hands.sort_by(|a, b| {
        let ordering = a.hand_type.cmp(&b.hand_type);
        if ordering != Ordering::Equal {
            return ordering;
        }

        for i in 0..5 {
            let o = cmp_cards_joker(a.cards.chars().nth(i).unwrap(), b.cards.chars().nth(i).unwrap());
            if o != Ordering::Equal {
                return o;
            }
        }

        return Ordering::Equal;
    });

    let sum = hands.iter()
        .enumerate()
        .fold(0, |acc: u32, (i, hand)| acc + hand.bid * (i as u32 + 1));

    return sum.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let result = calculate1("
                32T3K 765
                T55J5 684
                KK677 28
                KTJJT 220
                QQQJA 483
            "
        );
        assert_eq!(result, "6440")
    }

    #[test]
    fn test_2() {
        let result = calculate2("
                32T3K 765
                T55J5 684
                KK677 28
                KTJJT 220
                QQQJA 483
            "
        );
        assert_eq!(result, "5905")
    }
}