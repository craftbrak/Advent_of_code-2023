use common_utils::read_file;

fn main() {
    println!("p1: {}",part_one(read_file("day9/input").unwrap()));
    println!("p2: {}",part_two(read_file("day9/input").unwrap()));
}
fn part_one(input: String) -> i64 {
    let histories = parse_history(input);
    return histories.iter().map(|x| {find_next_val(x)}).sum();
}
fn part_two(input: String) -> i64 {
    let histories = parse_history(input);
    return histories.iter().map(|x| {find_previous_val(x)}).sum();
}
fn parse_history(string: String) ->Vec<Vec<i64>>{
    string.lines().collect::<Vec<&str>>().iter().map(|x| {x.split_whitespace().map(|x1| {x1.parse::<i64>().unwrap()}).collect()}).collect()
}
fn find_next_val(history: &Vec<i64>) ->i64{
    let mut history_sequences = get_history_sequences(history); // generate all the sequences for a given history
    for i in (1..history_sequences.len()).rev() {
        let bottom = history_sequences.get(i).unwrap().last().unwrap();
        let top = history_sequences.get(i-1).unwrap().last().unwrap();
        let next_val = bottom + top;
        if i-1 == 0 { return next_val }
        history_sequences.get_mut(i-1).unwrap().push(next_val)
    }
    return *history_sequences.first().unwrap().last().unwrap()
}
fn find_previous_val(history: &Vec<i64>) -> i64 {
    let mut history_sequences = get_history_sequences(history);
    for i in (1..history_sequences.len()).rev() {
        let bottom = history_sequences.get(i).unwrap().first().unwrap();
        let top = history_sequences.get(i-1).unwrap().first().unwrap();
        let next_val = top - bottom;
        if i-1 == 0 { return next_val }
        history_sequences.get_mut(i-1).unwrap().insert(0,next_val)
    }
    return *history_sequences.first().unwrap().first().unwrap()
}
fn get_history_sequences(history: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut history_sequences = Vec::new();
    history_sequences.push(history.clone());
    while !history_sequences.last().unwrap().iter().all(|x1| {*x1 == 0}) {
        let mut new_sequence = Vec::new();
        let iter = history_sequences.last().unwrap().iter().zip(history_sequences.last().unwrap().iter().skip(1)); //make a interator that return both the current and the previous number ?
        for (prev,cur) in iter {
            new_sequence.push(cur- prev)
        }
        history_sequences.push(new_sequence)
    }
    return history_sequences;
}
#[cfg(test)]
mod test{
    use common_utils::read_file;
    use crate::{find_next_val, part_one};

    #[test]
    fn  test_part1_exemple1(){
        let input = read_file("exemple").unwrap();
        let res = part_one(input);
        assert_eq!(res,114);
    }
    #[test]
    fn  test_next_val(){
        let input = vec![1i64,3,6,10,15];
        let res = find_next_val(&input);
        assert_eq!(res,21);
    }
    #[test]
    fn  test_next_val2(){
        let input = vec![10i64,13,16,21,30,45];
        let res = find_next_val(&input);
        assert_eq!(res,68);
    }
}