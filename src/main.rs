mod block;
mod block_chain;

fn main() -> Result<(), std::io::Error> {
    block_chain::run()
}
