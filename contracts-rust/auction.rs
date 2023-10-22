use std::thread;
use std::sync::mpsc;

// This struct represents a vid on an item in an auction.
struct Bid {
    amount: u64,
    bidder: string,
}

// This functions starts a thread that submits a bid to an auction.
fn submit_bid(tx: mpsc::Sender<Bid>, amount: u64, bidder: String) {
    let mut conn = web3::connect("http://localhost:8545");
    let auction_contract = conn.eth().contract(
        "0x1234567890abcdef1234567890abcdef1234567890",
    );

    let bid = Bid { amount, bidder };

    tx.send(bid).unwrap();
}

// This function starts a number of threads to submit bids to an auction.
fn main() {
    let (tx, rx) = mpsc::channel();
    
    for _ in 0..100 {
        thread::spawn(move || {
            submit_bid(tx, 100, "Alice".to_string());
        });
    }

    for bid in rx.iter() {
        println!("Received bid: {:?}", bid);
    }
}