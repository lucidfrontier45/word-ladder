use shiritori::agent::Agent;
use shiritori::io;
use shiritori::judge::JudgeResult;

fn main() {
    // get args
    let args: Vec<String> = std::env::args().collect();
    let token_list_file = &args[1];
    let tokens = io::read_token_list(token_list_file);
    let rule = shiritori::judge::Rule::with_invalid_last_chars(tokens, ['ン']);
    let mut judge = shiritori::judge::Judge::new(rule.clone());
    let mut agent = shiritori::agent::SimpleAgent::new(rule);

    // start game
    loop {
        // readline from stdin
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let player_token = input.trim();
        dbg!(player_token);

        let result = judge.make_judgement(player_token);
        match result {
            JudgeResult::Lose => {
                println!("You lose!");
                break;
            }
            JudgeResult::Continue => {}
        }

        // agent's turn
        let agent_token = agent.next_token(player_token).unwrap_or_default();
        dbg!(agent_token.as_str());
        let result = judge.make_judgement(agent_token.as_str());
        match result {
            JudgeResult::Lose => {
                println!("You win!");
                break;
            }
            JudgeResult::Continue => {}
        }
    }
}
