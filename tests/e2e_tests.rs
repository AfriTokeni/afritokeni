mod mocks;
mod e2e;

use cucumber::World;
use e2e::steps::world::UssdWorld;

#[tokio::main]
async fn main() {
    UssdWorld::cucumber()
        .run("e2e/features/")
        .await;
}
