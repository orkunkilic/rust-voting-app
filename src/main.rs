mod voting;
pub use crate::voting::structs::*;

fn main() {
    let mut voters = Vec::new();
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
