use candid::{CandidType, Deserialize};
use ic_cdk::api::call::ManualReply;
use ic_cdk_macros::update;
use junobuild_macros::{
    assert_delete_asset, assert_delete_doc, assert_set_doc, assert_upload_asset, on_delete_asset,
    on_delete_doc, on_delete_filtered_assets, on_delete_filtered_docs, on_delete_many_assets,
    on_delete_many_docs, on_set_doc, on_set_many_docs, on_upload_asset,
};
use junobuild_satellite::{
    include_satellite, AssertDeleteAssetContext, AssertDeleteDocContext, AssertSetDocContext,
    AssertUploadAssetContext, OnDeleteAssetContext, OnDeleteDocContext,
    OnDeleteFilteredAssetsContext, OnDeleteFilteredDocsContext, OnDeleteManyAssetsContext,
    OnDeleteManyDocsContext, OnSetDocContext, OnSetManyDocsContext, OnUploadAssetContext,
};
use serde::Serialize;

mod http_handlers;
mod ussd;
mod sms;

#[cfg(test)]
mod tests;

// All the available hooks and assertions for your Datastore and Storage are scaffolded by default in this `lib.rs` module.
// However, if you don’t have to implement all of them, for example to improve readability or reduce unnecessary logic,
// you can selectively enable only the features you need.
//
// To do this, disable the default features in your `Cargo.toml` and explicitly specify only the ones you want to use.
//
// For example, if you only need `on_set_doc`, configure your `Cargo.toml` like this:
//
// [dependencies]
// junobuild-satellite = { version = "0.0.22", default-features = false, features = ["on_set_doc"] }
//
// With this setup, only `on_set_doc` must be implemented with custom logic,
// and other hooks and assertions can be removed. They will not be included in your Satellite.

#[on_set_doc]
async fn on_set_doc(_context: OnSetDocContext) -> Result<(), String> {
    Ok(())
}

#[on_set_many_docs]
async fn on_set_many_docs(_context: OnSetManyDocsContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_doc]
async fn on_delete_doc(_context: OnDeleteDocContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_many_docs]
async fn on_delete_many_docs(_context: OnDeleteManyDocsContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_filtered_docs]
async fn on_delete_filtered_docs(_context: OnDeleteFilteredDocsContext) -> Result<(), String> {
    Ok(())
}

#[on_upload_asset]
async fn on_upload_asset(_context: OnUploadAssetContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_asset]
async fn on_delete_asset(_context: OnDeleteAssetContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_many_assets]
async fn on_delete_many_assets(_context: OnDeleteManyAssetsContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_filtered_assets]
async fn on_delete_filtered_assets(_context: OnDeleteFilteredAssetsContext) -> Result<(), String> {
    Ok(())
}

#[assert_set_doc]
fn assert_set_doc(_context: AssertSetDocContext) -> Result<(), String> {
    Ok(())
}

#[assert_delete_doc]
fn assert_delete_doc(_context: AssertDeleteDocContext) -> Result<(), String> {
    Ok(())
}

#[assert_upload_asset]
fn assert_upload_asset(_context: AssertUploadAssetContext) -> Result<(), String> {
    Ok(())
}

#[assert_delete_asset]
fn assert_delete_asset(_context: AssertDeleteAssetContext) -> Result<(), String> {
    Ok(())
}

/// Custom HTTP endpoint handler for USSD and SMS webhooks
/// 
/// This allows Africa's Talking to send webhooks directly to the Satellite
/// Routes:
/// - POST /api/ussd - USSD webhook handler
/// - POST /api/sms - SMS webhook handler
#[update(manual_reply = true)]
fn http_request_update(req: http_handlers::HttpRequest) -> ManualReply<http_handlers::HttpResponse> {
    http_handlers::route_request(req)
}

include_satellite!();
