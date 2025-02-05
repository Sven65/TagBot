use chrono::prelude::*;
use rlua::{Error as LuaError, FromLua, IntoLua, MetaMethod, UserData, Value};
use serenity::model::timestamp::{self, Timestamp};
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::{utils::types::ConstructableFrom, Requireable};

#[derive(Clone, Debug)]
#[lua_document("TBTimestamp", class)]
pub struct TBTimestamp(Timestamp);

impl FromLua<'_> for TBTimestamp {
	fn from_lua(value: Value<'_>, _lua: &'_ rlua::Lua) -> rlua::Result<Self> {
		let tb_timestamp = value.as_userdata();

		if tb_timestamp.is_none() {
			panic!("Passed value is none");
		}

		let tb_timestamp = tb_timestamp.unwrap();

		if !tb_timestamp.is::<TBTimestamp>() {
			return Err(LuaError::external("Passed type is not TBEmbed"));
		}

		let tb_timestamp = match tb_timestamp.take::<TBTimestamp>() {
			Ok(timestamp) => timestamp,
			Err(_) => return Err(LuaError::external("Failed to take internal TBEmbed")),
		};

		Ok(tb_timestamp)
	}
}

impl TBTimestamp {
	pub fn new(timestamp: Timestamp) -> TBTimestamp {
		TBTimestamp(timestamp)
	}
}

impl ConstructableFrom<Timestamp> for TBTimestamp {
	fn new(timestamp: Timestamp) -> TBTimestamp {
		TBTimestamp(timestamp)
	}
}

impl std::ops::Deref for TBTimestamp {
	type Target = Timestamp;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<TBTimestamp> for Timestamp {
	fn from(val: TBTimestamp) -> Self {
		val.0
	}
}

impl UserData for TBTimestamp {
	#[lua_document("TBTimestamp", tostring, parse_comments)]
	#[allow(unused_doc_comments)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		// methods.add_meta_method(MetaMethod::Index, )

		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			this.0.to_string().into_lua(ctx)
		});

		/// @desc Formats the timestamp with with the specified format string.
		/// @method
		/// @param {string} value The [format string](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) to use when formatting
		/// @return {string} Formatted timestamp
		methods.add_method("format", |ctx, this, value: String| {
			let time = Utc.timestamp(this.unix_timestamp(), 0);

			let formatted = time.format(&value);

			Ok(formatted.to_string().into_lua(ctx))
		});

		/// @desc Formats with discord timestamp tag (https://gist.github.com/LeviSnoot/d9147767abeef2f770e9ddcd91eb85aa)
		/// @method
		/// @param {string} specifier The [specifier](https://gist.github.com/LeviSnoot/d9147767abeef2f770e9ddcd91eb85aa) to use when formatting
		/// @return {string} Discord formatted timestamp
		methods.add_method("d_format", |_ctx, this, specifier: String| {
			let is_valid_specifier =
				["", "t", "T", "d", "D", "f", "F", "R"].contains(&specifier.as_str());

			if !is_valid_specifier {
				return Err(LuaError::external(format!(
					"Invalid format specifier `{}` provided",
					specifier
				)));
			}

			if specifier.is_empty() {
				return Ok(format!("<t:{}>", this.0.unix_timestamp()));
			}

			Ok(format!("<t:{}:{}>", this.0.unix_timestamp(), specifier))
		})
	}
}

#[lua_document("TBTimestamp", requireable = "timestamp")]
#[allow(unused_doc_comments)]
impl Requireable for TBTimestamp {
	/// @desc Creates a requireable module
	/// @return {module} The timestamp module
	fn create_module<'lua>(ctx: rlua::Context<'lua>) -> rlua::Value<'lua> {
		let value = ctx.create_table();

		if value.is_err() {
			return rlua::Nil;
		}

		let value = value.unwrap();

		/// @desc Creates a new timestamp
		/// @function
		/// @param {table} params The params to create the timestamp with
		/// @param {i64} params.secs The unix timestamp to create the timestamp from
		/// @return {TBTimestamp} The new timestamp
		let func = ctx.create_function(|_, params: rlua::Table| {
			let secs = params.get::<&str, i64>("secs")?;

			Ok(TBTimestamp::new(
				Timestamp::from_unix_timestamp(secs).unwrap(),
			))
		});

		value.set("new", func.unwrap()).unwrap();

		Value::Table(value.clone())
	}
}
