use rlua::{UserData, MetaMethod, ToLua, Value, FromLuaMulti};
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
			let time = DateTime::parse_from_str(s, fmt)

			time.

			Ok(())
		});
	}
}