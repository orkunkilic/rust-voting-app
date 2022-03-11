pub mod structs {
    pub struct Voter{
        id: u32,
        name: String,
        age: u8
    }

    impl Voter {
        pub fn new(id: u32, name: String, age: u8) -> Voter {
            Voter {
                id,
                name,
                age
            }
        }

        pub fn get_id(&self) -> u32 {
            self.id
        }

        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        pub fn is_eligible_to_vote(&self) -> bool {
            self.age >= 18
        }

    }
    
    pub struct Auction {
        id: u16,
        title: String,
        description: String,
        eligible_voter_ids: Vec<u32>,
        votes: Vec<Vote>,
        is_started: bool,
        is_ended: bool
    }

    impl Auction {
        pub fn new(id: u16, title: String, description: String) -> Auction {
            Auction {
                id,
                title,
                description,
                eligible_voter_ids: Vec::new(),
                votes: Vec::new(),
                is_started: false,
                is_ended: false
            }
        }

        pub fn get_id(&self) -> u16 {
            self.id
        }

        pub fn add_voters(&mut self, mut voter_ids: Vec<u32>) {
            self.eligible_voter_ids.append(&mut voter_ids);
        }

        pub fn start_auction(&mut self) {
            self.is_started = true;
        }

        pub fn end_auction(&mut self) {
            self.is_ended = true;
        }

        pub fn vote(&mut self, vote: Vote) {
            if self.is_started && !self.is_ended {
                self.votes.push(vote);
            }
        }

        pub fn get_votes(&self) -> Vec<Vote> {
            self.votes.clone()
        }

    }
    
    #[derive(Debug)]
    #[derive(Clone)]
    pub enum VoteType {
        UPVOTE,
        DOWNVOTE
    }
    
    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Vote{
        voter_id: u32,
        vote: VoteType
    }

    impl Vote {
        pub fn new(voter_id: u32, vote: VoteType) -> Vote {
            Vote {
                voter_id,
                vote
            }
        }

        pub fn get_voter_id(&self) -> u32 {
            self.voter_id
        }

        pub fn get_vote(&self) -> VoteType {
            self.vote.clone()
        }
    }
}
