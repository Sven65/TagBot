use rlua::{ToLua, UserData, MetaMethod, Value};
use serenity::model::{prelude::{Member}};

use super::timestamp::TBTimestamp;

/// Wrapper for Serenity Member
#[derive(Clone, Debug)]
pub struct TBMember(Member);

impl TBMember {
	pub fn new(member: Member) -> TBMember {
		TBMember(member)
	}
}


// This looks wild, but it's needed for indexing lol
// TODO: Make this use the new util functions
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
				&"nick" => {
					let nick = this.0.nick.as_ref();
					if nick.is_none() {
						Value::Nil
					} else {
						nick.unwrap().clone().to_lua(ctx)?
					}
				},
				// &"roles" => "",
				&"pending" => this.0.pending.to_lua(ctx)?,
				&"premium_since" => {
					let premium_since = this.0.premium_since;
					if premium_since.is_none() {
						Value::Nil
					} else {
						let tb_timestamp = TBTimestamp::new(premium_since.unwrap());
						tb_timestamp.to_lua(ctx)?
					}
				},
				// &"permissions" => {},
				&"avatar" => {
					let avatar = this.0.avatar.as_ref();
					if avatar.is_none() {
						Value::Nil
					} else {
						avatar.unwrap().clone().to_lua(ctx)?
					}
				},
				&"communication_disabled_until" => {
					let comms_disabled = this.0.communication_disabled_until;
					if comms_disabled.is_none() {
						Value::Nil
					} else {
						let tb_timestamp = TBTimestamp::new(comms_disabled.unwrap());
						tb_timestamp.to_lua(ctx)?
					}
				},
				&_ => Value::Nil,
			})
		});
	}
}