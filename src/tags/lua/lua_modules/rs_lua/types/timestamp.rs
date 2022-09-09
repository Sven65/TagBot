use rlua::{UserData, MetaMethod, ToLua, Value, Error as LuaError};
use serenity::model::timestamp::Timestamp;
use chrono::prelude::*;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct TBTimestamp(Timestamp);

impl TBTimestamp {
	pub fn new(timestamp: Timestamp) -> TBTimestamp {
		TBTimestamp(timestamp)
	}
}

impl std::ops::Deref for TBTimestamp {
	type Target = Timestamp;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl UserData for TBTimestamp {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		// methods.add_meta_method(MetaMethod::Index, )

		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			Ok(this.0.to_string().to_lua(ctx)?)
		});

		methods.add_method("format", |ctx, this, value: String| {
			let time = Utc.timestamp(this.unix_timestamp(), 0);

			let formatted = time.format(&value);

			Ok(formatted.to_string().to_lua(ctx))
		});

		// Formats with discord timestamp tag (https://gist.github.com/LeviSnoot/d9147767abeef2f770e9ddcd91eb85aa)
		methods.add_method("d_format", |_ctx, this, specifier: String| {
			let is_valid_specifier = ["", "t", "T", "d", "D", "f", "F", "R"].contains(&specifier.as_str());

			if !is_valid_specifier {
				return Err(LuaError::external(format!("Invalid format specifier `{}` provided", specifier)));
			}

			if specifier == "" {
				return Ok(format!("<t:{}>", this.0.unix_timestamp()));
			}
			
			return Ok(format!("<t:{}:{}>", this.0.unix_timestamp(), specifier));
		})
	}
}