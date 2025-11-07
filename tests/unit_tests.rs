// Unit tests runner using Cucumber BDD
// Run with: cargo test --test unit_tests

mod unit;

use cucumber::World;
use unit::steps::pin_steps::PinWorld;

#[tokio::main]
async fn main() {
    PinWorld::cucumber()
        .run("unit/features/")
        .await;
}
