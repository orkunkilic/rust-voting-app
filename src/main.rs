mod voting;
pub use crate::voting::structs::*;
use std::io::stdin;

fn main() {
    let mut voters = Vec::new();
    let mut auctions = Vec::new();

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
            let mut input = String::new();
            
            println!("Enter the title and description of the auction (title, description): ");
            stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let auction_array: Vec<&str> = input.trim().split(",").collect();

            let mut auction = add_auction(auctions.len().try_into().unwrap(), auction_array[0].to_string(), auction_array[1].trim().to_string());

            println!("Enter the voter ids (id1, id2, id3, ...): ");
            stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let voter_array: Vec<&str> = input.trim().split(",").collect();

            auction.add_voters(voter_array.iter().map(|x| x.trim().parse().unwrap()).collect());

            auctions.push(auction);

            println!("Auction added successfully");
        } else if (input == 3) {
            let mut input = String::new();
            
            println!("Vote for an auction as an user (auction_id, voter_id, UP|DOWN): ");
            stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let vote_array: Vec<&str> = input.trim().split(",").collect();

            // make this to get auction by id
            let result = give_vote(auctions[1], vote_array[1].trim().parse().unwrap(), vote_array[2].trim().to_string());

            if result {
                println!("Vote gived successfully");
            } else {
                println!("Vote failed");
            }
        }
    }
}

fn add_voter(id: u32, name: String, age: u8) -> Voter {
    let voter = Voter::new(id, name, age);
    return voter;
}

fn add_auction(id: u16, title: String, description: String) -> Auction {
    let auction = Auction::new(id, title, description);
    return auction;
}

fn give_vote(auction: Auction, voter_id: u32, vote: String) -> bool {
    let enum_vote = match vote.as_str() {
        "UP" => VoteType::UPVOTE,
        "DOWN" => VoteType::DOWNVOTE,
    };
    let vote = Vote::new(voter_id, enum_vote);

    return auction.vote(vote);
}

