mod graph;
mod profile_transactions;

#[macro_use]
extern crate lazy_static;

fn main() {
    println!("Hello, world!");
    // Call the function to test if the Bitcoin RPC is working
    let start_height = 378730; // Set appropriate start height
    let end_height = 378740;  // Set appropriate end height
    let graph = profile_transactions::build_transaction_graph(start_height, end_height);
    println!("Transaction graph built successfully: {:?}", graph);
    
}
