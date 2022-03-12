mod voting;
pub use crate::voting::structs::*;
use std::io::stdin;

fn main() {
    let mut voters = Vec::new();
    //  let mut auctions = Vec::new();

    loop {
        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if (input == 0) {
            break;
        } else if (input == 1) {
            let mut input = String::new();
            
            println!("Enter the name and age of the voter (name, age): ");
            stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input: Vec<&str> = input.trim().split(",").collect();

            voters.push(add_voter(voters.len().try_into().unwrap() ,input[0].to_string(), input[1].trim().parse().unwrap()));

            println!("Voter added successfully");
        } else if (input == 2) {

        } else if (input == 3) {

        }
    }
    for i in 1..6{
        let voter = Voter::new(i, "John".to_string(), 18);
        voters.push(voter);
    }
    let mut auction = Auction::new(1, String::from("Auction 1"), String::from("Auction 1 description"));
    auction.add_voters(vec![1, 2, 3, 4, 5]);
    auction.start_auction();
    for i in 1..6{
        let voter = &voters[i-1];
        if voter.is_eligible_to_vote() {
            println!("{} is eligible to vote", voter.get_name());
        }
    }
    for voter in voters {
        let vote = Vote::new(voter.get_id(), if voter.get_id() % 2 == 0 {VoteType::UPVOTE} else {VoteType::DOWNVOTE});
        auction.vote(vote);
    }
    println!("{:?}", auction.get_votes());
}

fn add_voter(id: u32, name: String, age: u8) -> Voter {
    let voter = Voter::new(id, name, age);
    return voter;
}


