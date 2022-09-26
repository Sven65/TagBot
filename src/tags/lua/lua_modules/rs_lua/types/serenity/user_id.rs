use rlua::{UserData, MetaMethod, Value, ToLua, Error as LuaError};
use serenity::model::prelude::{UserId};
use serenity::model::user::User;
use serenity::{prelude::{Context as SerenityContext}, Error};
use tokio::runtime::Handle;

use crate::tags::lua::lua_modules::rs_lua::types::utils::types::ConstructableFrom2;

use super::user::TBUser;

async fn get_user(user_id: UserId, s_ctx: SerenityContext) -> Result<User, Error> {
	let user = user_id.to_user(&s_ctx.http).await.unwrap();

	Ok(user)
}

/// Wrapper for [`serenity::model::prelude::UserId`]
#[derive(Clone)]
pub struct TBUserId(pub UserId, pub SerenityContext);

impl TBUserId {
	/// Creates a new wrapper
	/// 
	/// # Arguments
	/// * `user_id` - The serenity UserId to wrap
	/// * `s_ctx` - SerenityContext to use when resolving channel
	pub fn new(user_id: UserId, s_ctx: SerenityContext) -> TBUserId {
		TBUserId(user_id, s_ctx)
	}
}

impl ConstructableFrom2<UserId, SerenityContext> for TBUserId {
	/// Creates a new wrapper
	/// 
	/// # Arguments
	/// * `user_id` - The serenity UserId to wrap
	/// * `s_ctx` - SerenityContext to use when resolving channel
	fn new(user_id: UserId, s_ctx: SerenityContext) -> TBUserId {
		TBUserId(user_id, s_ctx)
	}
}

impl UserData for TBUserId {
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			Ok(this.0.to_string().to_lua(ctx)?)
		});

		methods.add_method("resolve", |ctx, this, _: Value| {
			let user_id = this.0.clone();
			let s_ctx = this.1.clone();

			let user = tokio::task::block_in_place(move || {
				return Handle::current().block_on(async move {
					let guild = get_user(user_id, s_ctx).await;

					return guild
				});
			});

			if user.is_err() {
				return Err(LuaError::external("Failed to get user."));
			}

			Ok(TBUser(user.unwrap()).to_lua(ctx)?)
		});
	}
}