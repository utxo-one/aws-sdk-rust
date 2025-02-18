#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>Amazon Web Services Support App in Slack</fullname>
//! <p>You can use the Amazon Web Services Support App in Slack API to manage your support cases in Slack for your
//! Amazon Web Services account. After you configure your Slack workspace and channel with the Amazon Web Services Support App, you can
//! perform the following tasks directly in your Slack channel:</p>
//! <ul>
//! <li>
//! <p>Create, search, update, and resolve your support cases</p>
//! </li>
//! <li>
//! <p>Request service quota increases for your account</p>
//! </li>
//! <li>
//! <p>Invite Amazon Web Services Support agents to your channel so that you can chat directly about your
//! support cases</p>
//! </li>
//! </ul>
//! <p>For more information about how to perform these actions in Slack, see the following
//! documentation in the <i>Amazon Web Services Support User Guide</i>:</p>
//! <ul>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/awssupport/latest/user/aws-support-app-for-slack.html">Amazon Web Services Support App in Slack</a>
//! </p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/awssupport/latest/user/joining-a-live-chat-session.html">Joining a live chat session with Amazon Web Services Support</a>
//! </p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/awssupport/latest/user/service-quota-increase.html">Requesting service quota increases</a>
//! </p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/awssupport/latest/user/support-app-commands.html">Amazon Web Services Support App commands in Slack</a>
//! </p>
//! </li>
//! </ul>
//! <p>You can also use the Amazon Web Services Management Console instead of the Amazon Web Services Support App API to manage your Slack
//! configurations. For more information, see <a href="https://docs.aws.amazon.com/awssupport/latest/user/authorize-slack-workspace.html">Authorize a
//! Slack workspace to enable the Amazon Web Services Support App</a>.</p>
//!
//! <note>
//! <ul>
//! <li>
//! <p>You must have a Business or Enterprise Support plan to use the Amazon Web Services Support App API. </p>
//! </li>
//! <li>
//! <p>For more information about the Amazon Web Services Support App endpoints, see the <a href="https://docs.aws.amazon.com/general/latest/gr/awssupport.html#awssupport_app_region">Amazon Web Services Support App in Slack endpoints</a> in the <i>Amazon Web Services General
//! Reference</i>.</p>
//! </li>
//! </ul>
//! </note>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
pub mod client;
/// Configuration for the service.
pub mod config;
/// All error types that operations can return.
pub mod error;
mod error_meta;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Generated accessors for nested fields
pub mod lens;
pub mod middleware;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Paginators for the service
pub mod paginator;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::result::SdkError;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("supportapp", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
