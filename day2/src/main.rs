use std::fs;

const FILE_NAME: &'static str = "input.txt";

fn main() {
    let file = fs::read_to_string(FILE_NAME)
        .expect("should have been able to read the file");
        
    let lines = file.split('\n');

    let mut total_score: u32 = 0;

    for l in lines {
        if l.is_empty() {
            break;
        }

        let opponent_hand = l.chars().nth(0).unwrap();
        let our_hand = l.chars().nth(2).unwrap();

        let outcome = did_win(&opponent_hand, &our_hand);
        let outcome_score = score_round_outcome(&outcome);
        let hand_score = score_hand(&our_hand);

        total_score = total_score + outcome_score + hand_score;

        println!("{} {}  -  outcome: {:?}, outcome_score: {}, hand_score: {}", opponent_hand, our_hand, outcome, outcome_score, hand_score);
    }

    println!("total score: {}", total_score)
}

#[derive(Debug)]
enum RoundOutcome {
    WIN,
    TIE,
    LOSE,
}

fn did_win(opp: &char, us: &char) -> RoundOutcome {
    if opp == &'A' && us == &'X'
        || opp == &'B' && us == &'Y'
        || opp == &'C' && us == &'Z'
    {
        return RoundOutcome::TIE;
    }

    if opp.eq(&'A') && us.eq(&'Y')
        || opp.eq(&'B') && us.eq(&'Z')
        || opp.eq(&'C') && us.eq(&'X')
    {
        return RoundOutcome::WIN;
    } 

    return RoundOutcome::LOSE;
}

fn score_hand(hand: &char) -> u32 {
    match hand {
        &'X' => 1,
        &'Y' => 2,
        &'Z' => 3,
        _ => 0
    }
}

fn score_round_outcome(outcome: &RoundOutcome) -> u32 {
    match outcome {
        RoundOutcome::WIN => 6,
        RoundOutcome::TIE => 3,
        RoundOutcome::LOSE => 0,
    }
}
