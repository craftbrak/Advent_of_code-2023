use std::cmp::Ordering;
use std::collections::{HashMap};
use common_utils::read_file;

#[derive(PartialOrd, Ord, PartialEq, Eq ,Debug)]
enum Kind{
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
#[derive(PartialOrd, Ord, PartialEq, Eq ,Debug, Hash)]
enum Card{
    Joker,
    Two,
    Tree,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace
}
#[derive(Eq, PartialEq, Debug)]
struct Hand{
    card:   [Card; 5],
    bid:    u64,
    kind:  Kind
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Custom comparison logic
        // For example, comparing based on field1:
        let res =self.kind.cmp(&other.kind);
        match res {
            Ordering::Less => {res}
            Ordering::Equal => {
                let mut subres = Ordering::Equal;
                for (i, card) in self.card.iter().enumerate() {
                    subres = card.cmp(&other.card[i]);
                    if subres == Ordering::Greater || subres== Ordering::Less { 
                        break
                    }
                }
                subres
            }
            Ordering::Greater => {res}
        }
        // You can add more complex logic here
    }
}
fn main() {
    let input = read_file("day7/input").unwrap_or("".to_string());
    println!("Part 1 result: {}\n Part 2 result: {} ",part_one(input.clone()),part_two(input));
}
fn part_one(input: String) -> u64 {
    let mut cards: Vec<Hand> = input.lines().map(|st: &str| {
        return  parse_hand(st);
    }).collect();
    cards.sort();
    println!("{:#?}",cards);
    return cards.iter().enumerate().map(|x| { x.1.bid* (x.0+1) as u64 }).sum();
}
fn part_two(input: String) -> u64 {
    return 0;
}
fn parse_hand(hand_str: &str)->Hand{
    let chars: [Card; 5] =hand_str.split_whitespace().next().unwrap().chars().map(|x1| {char_to_card(x1)})
        .collect::<Vec<Card>>().try_into()
        .expect("String did not have exactly 5 characters");
    let bid = hand_str.split_whitespace().nth(1).unwrap_or("1").parse::<u64>().unwrap_or(1);

    let mut counts = HashMap::new();
    for card in &chars {
        *counts.entry(card).or_insert(0) += 1;
    }
    let joker_count = *counts.get(&Card::Joker).unwrap_or(&0);
    counts.remove(&Card::Joker);
    let kind = match counts.len() {
        1 => Kind::FiveOfAKind,
        2 => {
            let &max_count = counts.values().max().unwrap();
            let kid =match max_count {
                3 => {
                    if joker_count == 1 {
                        Kind::FourOfAKind
                    } else {
                        Kind::FullHouse
                    }
                },
                2 => {
                    if joker_count == 2 {
                        Kind::FourOfAKind
                    } else {
                        Kind::FullHouse
                    }
                },
                _ => Kind::FourOfAKind,
            };
            kid
        }
        3 => {
            let &max_count = counts.values().max().unwrap();
            match joker_count {
                0 => {
                    if max_count == 3 {
                        Kind::ThreeOfAKind
                    } else {
                        Kind::TwoPair
                    }
                },
                1 => {
                    // Joker can turn a TwoPair into a FullHouse
                    if max_count == 2 {
                        Kind::FullHouse
                    } else {
                        // Joker doesn't change a ThreeOfAKind
                        Kind::ThreeOfAKind
                    }
                },
                _ => Kind::ThreeOfAKind, // Multiple Jokers still result in ThreeOfAKind in this scenario
            }
        }
        4 => {
            if joker_count > 0 {
                Kind::ThreeOfAKind
            } else {
                Kind::OnePair
            }
        },
        5 => {Kind::HighCard}
        _ => {Kind::FiveOfAKind}
    };
    return Hand{card: chars,bid,kind }
}
fn char_to_card(car: char)->Card{
    return match car {
        '2'=> {Card::Two},
        '3'=> {Card::Tree},
        '4'=> {Card::Four},
        '5'=> {Card::Five},
        '6'=> {Card::Six},
        '7'=> {Card::Seven},
        '8'=> {Card::Eight},
        '9'=> {Card::Nine},
        'T'=> {Card::Ten},
        'J'=> {Card::Joker },
        'Q'=> {Card::Queen },
        'K'=> {Card::King },
        'A'=> {Card::Ace },
        _ => {Card::Joker}
    }
}
#[cfg(test)]
mod test_parse_hand{
    use crate::{Card, Kind, parse_hand};

    #[test]
    fn test_parse_hand1(){
        let hand = parse_hand("72772 82");
        assert_eq!(hand.bid, 82);
        assert_eq!(hand.kind, Kind::FullHouse);
        assert_eq!(hand.card, [Card::Seven,Card::Two,Card::Seven,Card::Seven,Card::Two]);
    }
    #[test]
    fn test_parse_hand2() {
        let hand = parse_hand("8Q278 230");
        assert_eq!(hand.bid, 230);
        assert_eq!(hand.card, [Card::Eight, Card::Queen, Card::Two, Card::Seven, Card::Eight]);
        assert_eq!(hand.kind, Kind::OnePair);
    }

    #[test]
    fn test_parse_hand3() {
        let hand = parse_hand("QQJQQ 42");
        assert_eq!(hand.bid, 42);
        assert_eq!(hand.card, [Card::Queen, Card::Queen, Card::Joker, Card::Queen, Card::Queen]);
        assert_eq!(hand.kind, Kind::FiveOfAKind);
    }

    #[test]
    fn test_parse_hand4() {
        let hand = parse_hand("77777 148");
        assert_eq!(hand.bid, 148);
        assert_eq!(hand.card, [Card::Seven, Card::Seven, Card::Seven, Card::Seven, Card::Seven]);
        assert_eq!(hand.kind, Kind::FiveOfAKind);
    }

    #[test]
    fn test_parse_hand5() {
        let hand = parse_hand("8AJ8A 528");
        assert_eq!(hand.bid, 528);
        assert_eq!(hand.card, [Card::Eight, Card::Ace, Card::Joker, Card::Eight, Card::Ace]);
        assert_eq!(hand.kind, Kind::FullHouse);
    }

    #[test]
    fn test_parse_hand6() {
        let hand = parse_hand("87A6K 976");
        assert_eq!(hand.bid, 976);
        assert_eq!(hand.card, [Card::Eight, Card::Seven, Card::Ace, Card::Six, Card::King]);
        assert_eq!(hand.kind, Kind::HighCard);
    }

    #[test]
    fn test_parse_hand7() {
        let hand = parse_hand("TTTT5 957");
        assert_eq!(hand.bid, 957);
        assert_eq!(hand.card, [Card::Ten, Card::Ten, Card::Ten, Card::Ten, Card::Five]);
        assert_eq!(hand.kind, Kind::FourOfAKind);
    }

    #[test]
    fn test_parse_hand8() {
        let hand = parse_hand("QJAQA 704");
        assert_eq!(hand.bid, 704);
        assert_eq!(hand.card, [Card::Queen, Card::Joker, Card::Ace, Card::Queen, Card::Ace]);
        assert_eq!(hand.kind, Kind::FullHouse);
    }

}

#[cfg(test)]
mod test_part_one{
    use crate::{part_one};

    #[test]
    fn test_part_one(){
        let res = part_one("32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483".to_string());
        assert_eq!(res, 5905);
    }
}