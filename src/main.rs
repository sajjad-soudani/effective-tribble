use effective_tribble::set_done;
use sea_orm::*;
use futures::executor::block_on;

async fn run() -> Result<(), DbErr> {
    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
