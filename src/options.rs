//! Utilities for handling slash command options.

use std::collections::HashMap;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::application::interaction::application_command::CommandDataOption;
use serenity::model::user::User;
use serenity::model::channel::PartialChannel;
use serenity::model::guild::Role;
use serenity::model::channel::Attachment;
use crate::error::{AngyError, AngyResult};


/// A [`HashMap`] of [`CommandDataOptionValue`].
pub type OptionsHashMap = HashMap<String, CommandDataOptionValue>;


/// Convert a [`Vec`] of [`CommandDataOption`] in a more manageable [`HashMap`] mapping option names to resolved values.
///
/// Optional values won't be present in the final [`HashMap`].
pub fn options_hashmap(options: Vec<CommandDataOption>) -> OptionsHashMap {
    options.into_iter().filter(|o| o.resolved.is_some()).map(|o| (o.name, o.resolved.unwrap())).collect()
}


fn option_required<'hm>(hashmap: &'hm OptionsHashMap, name: &str) -> AngyResult<&'hm CommandDataOptionValue> {
    hashmap.get(name).ok_or(AngyError::UserError("A required option is missing."))
}

fn option_optional<'hm>(hashmap: &'hm OptionsHashMap, name: &str) -> Option<&'hm CommandDataOptionValue> {
    hashmap.get(name)
}

macro_rules! option_required {
    ($fn_name:ident, $type:ty, $kind:path) => {
        pub fn $fn_name<'hm>(hashmap: &'hm OptionsHashMap, name: &str) -> AngyResult<&'hm $type> {
            match option_required(hashmap, name)? {
                $kind(a, ..) => Ok(a),
                _            => panic!("Tried to parse an option with the wrong type")
            }
        }
    }
}

macro_rules! option_optional {
    ($fn_name:ident, $type:ty, $kind:path) => {
        pub fn $fn_name<'hm>(hashmap: &'hm OptionsHashMap, name: &str) -> Option<&'hm $type> {
            option_optional(hashmap, name).map(|v| match v {
                $kind(a, ..) => a,
                _            => panic!("Tried to parse an option with the wrong type")
            })
        }
    }
}

macro_rules! option {
    ($fn_req:ident, $fn_opt:ident, $type:ty, $kind:path) => {
        option_required!($fn_req, $type, $kind);
        option_optional!($fn_opt, $type, $kind);
    }
}

option!(option_required_string,     option_optional_string,     String,         CommandDataOptionValue::String);
option!(option_required_integer,    option_optional_integer,    i64,            CommandDataOptionValue::Integer);
option!(option_required_boolean,    option_optional_boolean,    bool,           CommandDataOptionValue::Boolean);
option!(option_required_user,       option_optional_user,       User,           CommandDataOptionValue::User);
option!(option_required_channel,    option_optional_channel,    PartialChannel, CommandDataOptionValue::Channel);
option!(option_required_role,       option_optional_role,       Role,           CommandDataOptionValue::Role);
option!(option_required_number,     option_optional_number,     f64,            CommandDataOptionValue::Number);
option!(option_required_attachment, option_optional_attachment, Attachment,     CommandDataOptionValue::Attachment);