use word_ladder::agent::Agent;
use word_ladder::judge::JudgeResult;
use word_ladder::{agent, io, judge};

fn main() {
    // get args
    let args: Vec<String> = std::env::args().collect();
    let token_list_file = &args[1];
    let tokens = io::read_token_list(token_list_file);
    let rule = judge::Rule::with_invalid_last_chars(tokens, ['ン']);
    let mut judge = judge::Judge::new(rule.clone());
    let mut agent = agent::SimpleAgent::new(rule);
    let mut previous_token: Option<String> = None;
    // start game
    loop {
        // readline from stdin
        let mut player_token = String::new();
        std::io::stdin().read_line(&mut player_token).unwrap();
        player_token = player_token.trim().to_string();
        dbg!(&player_token);

        let result = judge.make_judgement(player_token.as_str(), previous_token);
        match result {
            JudgeResult::Lose => {
                println!("You lose!");
                break;
            }
            JudgeResult::Continue => {}
        }
        previous_token = Some(player_token);

        // agent's turn
        let agent_token = agent
            .next_token(previous_token.as_ref().unwrap())
            .unwrap_or_default();
        dbg!(agent_token.as_str());
        let result = judge.make_judgement(agent_token.as_str(), previous_token);
        match result {
            JudgeResult::Lose => {
                println!("You win!");
                break;
            }
            JudgeResult::Continue => {}
        }
        previous_token = Some(agent_token);
    }
}
