use std::collections::{BinaryHeap, HashMap, HashSet};

const FEED_LENGTH: usize = 10;

struct Twitter {
    follows: HashMap<i32, HashSet<i32>>,
    tweets: HashMap<i32, Vec<Tweet>>, // (i32, u32) -> (tweet_id, glob_tweet_counter)
    glob_tweet_counter: u64,
}

#[derive(Clone, Copy, Debug)]
struct Tweet {
    id: i32,
    glob_count: u64,
}

impl Ord for Tweet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.glob_count.cmp(&other.glob_count)
    }
}

impl PartialOrd for Tweet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.glob_count.cmp(&other.glob_count))
    }
}

impl Eq for Tweet {}

impl PartialEq for Tweet {
    fn eq(&self, other: &Self) -> bool {
        self.glob_count == other.glob_count
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Twitter {
            follows: HashMap::new(),
            tweets: HashMap::new(),
            glob_tweet_counter: 0,
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let tw = Tweet {
            id: tweet_id,
            glob_count: self.glob_tweet_counter,
        };

        // println!("post_tweet: {} | {:?}", user_id, tw);

        match self.tweets.get_mut(&user_id) {
            None => {
                self.tweets.insert(user_id, vec![tw]);
            }
            Some(v) => {
                v.push(tw);
            }
        };
        self.glob_tweet_counter += 1;
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut posters = match self.follows.get(&user_id) {
            Some(s) => s.iter().cloned().collect::<Vec<i32>>(),
            None => {
                vec![]
            }
        };
        posters.push(user_id);

        // println!(
        //     "get_news_feed - user_id: {}, followees: {:?}",
        //     user_id, posters
        // );

        let mut feed = BinaryHeap::<Tweet>::new();
        for f_id in posters.iter() {
            let tweets = match self.tweets.get(f_id) {
                None => {
                    continue;
                }
                Some(v) => v.clone(),
            };

            for t in tweets {
                // println!("feed pushed {:?}", t);
                feed.push(t)
            }
        }

        let mut res = Vec::<i32>::new();

        for _ in 0..FEED_LENGTH {
            match feed.pop() {
                None => {
                    break;
                }
                Some(t) => {
                    // println!("feed pop {:?}", t);
                    res.push(t.id);
                }
            }
        }

        res
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        match self.follows.get_mut(&follower_id) {
            None => {
                let mut s = HashSet::<i32>::new();
                s.insert(followee_id);

                self.follows.insert(follower_id, s);
            }
            Some(s) => {
                s.insert(followee_id);
            }
        };
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        match self.follows.get_mut(&follower_id) {
            None => {}
            Some(s) => {
                s.remove(&followee_id);
            }
        }
    }
}
