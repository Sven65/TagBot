use rlua::{MetaMethod, ToLua, UserData, Value};
use serenity::utils::Colour;

use crate::tags::lua::lua_modules::rs_lua::types::{
	utils::{functions::convert_type, types::ConstructableFrom},
	Requireable,
};

/// Wrapper for [`serenity::utils::Colour`]
#[derive(Clone)]
pub struct TBColour(pub Colour);

impl ConstructableFrom<Colour> for TBColour {
	/// Creates a new wrapper
	///
	/// # Arguments
	/// * `colour` - The serenity Colour to wrap
	fn new(colour: Colour) -> TBColour {
		TBColour(colour)
	}
}

impl UserData for TBColour {
	#[rustfmt::skip]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			this.0.hex().to_string().to_lua(ctx)
		});

		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"r" => convert_type::<u8>(this.0.r(), ctx)?,
				"g" => convert_type::<u8>(this.0.g(), ctx)?,
				"b" => convert_type::<u8>(this.0.b(), ctx)?,
				_ => Value::Nil,
			})
		});

		methods.add_method("hex", |ctx, this, _: Value| {
			this.0.hex().to_lua(ctx)
		});
	}
}

impl Requireable for TBColour {
	fn create_module(ctx: rlua::Context) -> rlua::Value {
		let value = ctx.create_table();

		if value.is_err() {
			return rlua::Nil;
		}

		let value = value.unwrap();

		let func = ctx.create_function(|_, params: u32| {
			let value = params;

			Ok(TBColour::new(Colour::new(value)))
		});

		value.set("new", func.unwrap()).unwrap();

		let from_rgb = ctx.create_function(|_, (r, g, b): (u8, u8, u8)| {
			Ok(TBColour::new(Colour::from_rgb(r, g, b)))
		});

		value.set("from_rgb", from_rgb.unwrap()).unwrap();

		Value::Table(value.clone())
	}
}
