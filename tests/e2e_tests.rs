mod mocks;
mod steps;

use cucumber::World;
use steps::world::UssdWorld;

#[tokio::main]
async fn main() {
    UssdWorld::cucumber()
        .run("tests/features/")
        .await;
}
