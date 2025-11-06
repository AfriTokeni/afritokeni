// Unit tests runner using Cucumber BDD
// Run with: cargo test --test unit_tests

mod steps {
    pub mod pin_steps;
}

use cucumber::World;
use steps::pin_steps::PinWorld;

#[tokio::main]
async fn main() {
    PinWorld::cucumber()
        .run("tests/unit/features/")
        .await;
}
