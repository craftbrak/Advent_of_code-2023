use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use common_utils::read_file;
use crate::Kind::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

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
    println!("Part 1 result: {}\nPart 2 result: {} ",part_one(input.clone()),part_two(input));
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
    let mut cards: Vec<Hand> = input.lines().map(|st: &str| {
        return parse_hand_part_two(st);
    }).collect::<Vec<Hand>>();
    cards.sort();
    println!("{:#?}",cards);
    return cards.iter().enumerate().map(|x| { x.1.bid* (x.0+1) as u64 }).sum();
}
fn parse_hand(hand_str: &str)->Hand{
    let chars: [Card; 5] =match hand_str.split_whitespace().collect::<Vec<&str>>()[0].chars().map(|x1| {char_to_card(x1)}).collect::<Vec<Card>>().try_into(){
        Ok(arr)=> arr,
        Err(_)=> panic!("String did not have exactly 5 characters")
    };
    let bid = hand_str.split_whitespace().collect::<Vec<&str>>()[1].parse::<u64>().unwrap_or(1);

    let types: HashSet<&Card> = HashSet::from_iter(chars.iter().clone());
    let kind= match types.len() {
        1 => {Kind::FiveOfAKind}
        2 => {
            let mut typ=Kind::FullHouse;
            let count =chars.iter().filter(|x| {**x== chars[0] }).count();
            if count == 4 || count == 1 {typ = Kind::FourOfAKind}
            typ
        }// 4 les mm 1 dif FourOfAKind et 3 les mm 2 autre mais les mm  FullHouse
        3 => {
            let mut typ :Kind = Kind::TwoPair;
            for c in types {
                let count =chars.iter().filter(|x| {*x == c }).count();
                if count == 3{typ = Kind::ThreeOfAKind;break}
                if count == 2{typ = Kind::TwoPair;break}
            }
            typ
        } // 3 les mm 2 diff TreeOfAKind et 2 les mm + 2 autre mm + 1 diff TwoPair
        4 => { Kind::OnePair}
        _ => {Kind::HighCard}
    };


    return Hand{card: chars,bid,kind }
}
fn parse_hand_part_two(hand_str: &str)->Hand{
    let chars: [Card; 5] =hand_str.split_whitespace().next().unwrap().chars().map(|x1| {char_to_card(x1)})
        .collect::<Vec<Card>>().try_into()
        .expect("String did not have exactly 5 characters");
    let bid = hand_str.split_whitespace().nth(1).unwrap_or("1").parse::<u64>().unwrap_or(1);

    // 1=> Five
    // 2 => avec J Five sans four / full
    // 3 => avec J four(max =3 || J >=2 ) / full sans three Two Pair
    // 4 => avec J three sans onePair
    // 5 => avec J Pair sans High

    let mut counts = HashMap::new();
    for card in &chars {
        *counts.entry(card).or_insert(0) += 1;
    }
    let joker_count = *counts.get(&Card::Joker).unwrap_or(&0);
    let kind = match counts.len() {
        1 => FiveOfAKind,
        2 => if joker_count > 0 { FiveOfAKind } else { if *counts.values().max().unwrap_or(&0) == 4 {FourOfAKind} else { FullHouse } }
        3 => {
            if joker_count > 0 {
                if *counts.values().max().unwrap_or(&0) == 3 || joker_count == 2 { FourOfAKind } else { FullHouse } }
            else {
                if *counts.values().max().unwrap_or(&0) == 3 { ThreeOfAKind } else { TwoPair } } }
        4 => if joker_count > 0  { ThreeOfAKind } else { OnePair }
        5 => if joker_count > 0 { OnePair } else { HighCard }
        _ => HighCard,
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
    use crate::{Card, Kind, parse_hand, parse_hand_part_two};

    #[test]
    fn test_parse_hand1(){
        let hand = parse_hand("72772 82");
        assert_eq!(hand.bid, 82);
        assert_eq!(hand.kind, Kind::FullHouse);
        assert_eq!(hand.card, [Card::Seven,Card::Two,Card::Seven,Card::Seven,Card::Two]);

        let hand = parse_hand_part_two("72772 82");
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

        let hand = parse_hand_part_two("8Q278 230");
        assert_eq!(hand.bid, 230);
        assert_eq!(hand.card, [Card::Eight, Card::Queen, Card::Two, Card::Seven, Card::Eight]);
        assert_eq!(hand.kind, Kind::OnePair);
    }

    #[test]
    fn test_parse_hand3() {
        let hand = parse_hand_part_two("QQJQQ 42");
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
        let hand = parse_hand_part_two("77777 148");
        assert_eq!(hand.bid, 148);
        assert_eq!(hand.card, [Card::Seven, Card::Seven, Card::Seven, Card::Seven, Card::Seven]);
        assert_eq!(hand.kind, Kind::FiveOfAKind);
    }

    #[test]
    fn test_parse_hand5() {
        let hand = parse_hand_part_two("8AJ8A 528");
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
        let hand = parse_hand_part_two("87A6K 976");
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
        let hand = parse_hand_part_two("TTTT5 957");
        assert_eq!(hand.bid, 957);
        assert_eq!(hand.card, [Card::Ten, Card::Ten, Card::Ten, Card::Ten, Card::Five]);
        assert_eq!(hand.kind, Kind::FourOfAKind);
    }

    #[test]
    fn test_parse_hand8() {
        let hand = parse_hand_part_two("QJAQA 704");
        assert_eq!(hand.bid, 704);
        assert_eq!(hand.card, [Card::Queen, Card::Joker, Card::Ace, Card::Queen, Card::Ace]);
        assert_eq!(hand.kind, Kind::FullHouse);
    }

}

#[cfg(test)]
mod test_part{
    use crate::{part_one, part_two};

    #[test]
    fn test_part_one(){
        let res = part_one("32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483".to_string());
        assert_eq!(res, 6440);
    }
    #[test]
    fn test_part_two(){
        let res = part_two("32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483".to_string());
        assert_eq!(res, 5905);
    }
}