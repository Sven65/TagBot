use rlua::{ToLua, UserData, MetaMethod, Value};
use serenity::model::{prelude::{User, Member}};

use super::timestamp::TBTimestamp;

#[derive(Clone, Debug)]
pub struct TBMember(Member);

impl TBMember {
	pub fn new(member: Member) -> TBMember {
		TBMember(member)
	}
}


// This looks wild, but it's needed for indexing lol
impl UserData for TBMember {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match &value.as_str() {
				&"deaf" => this.0.deaf.to_lua(ctx)?,
				// &"guild_id" => {},
				&"joined_at" => {
					let joined_at = this.0.joined_at;

					if joined_at.is_none() {
						Value::Nil
					} else {
						let tb_timestamp = TBTimestamp::new(joined_at.unwrap());

						tb_timestamp.to_lua(ctx)?

					}
				},
				&"mute" => this.0.mute.to_lua(ctx)?,
				// &"nick" => {},
				// &"roles" => "",
				&"pending" => this.0.pending.to_lua(ctx)?,
				// &"premium_since" => {},
				// &"permissions" => {},
				// &"avatar" => {},
				// &"communication_disabled_until" => {},
				&_ => Value::Nil,
			})
		});
	}
}