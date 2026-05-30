//! PostgreSQL-backed entity definitions for persistent memory storage.
//!
//! This module contains all database entities grouped by domain:
//!
//! - **Object storage**: [`bucket`], [`object_storage`] — S3-compatible blob storage references
//! - **Conversations**: [`conversation`], [`conversation_message`], [`conversation_content`] — chat history
//! - **Calendar**: [`calender`], [`calender_event`], [`calender_daily_event`], [`calender_task`] — scheduling
//! - **Contacts**: [`contact`], [`contact_identity`], [`contact_stories`] — relationship tracking
//! - **Diary**: [`diary`] — daily journal entries

pub mod bucket;
pub mod calender;
pub mod calender_daily_event;
pub mod calender_event;
pub mod calender_task;
pub mod contact;
pub mod contact_identity;
pub mod contact_stories;
pub mod conversation;
pub mod conversation_content;
pub mod conversation_message;
pub mod diary;
pub mod object_storage;