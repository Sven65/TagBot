use rlua::{MetaMethod, ToLua, UserData, Value};
use serenity::utils::Colour;
use tagbot_macros::lua_document;

use crate::tags::lua::lua_modules::rs_lua::types::{
	utils::{functions::convert_type, types::ConstructableFrom},
	Requireable,
};

/// Wrapper for [`serenity::utils::Colour`]
#[derive(Clone)]
#[lua_document("TBColour", class)]
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
	#[allow(unused_doc_comments)]
	#[lua_document("TBColour", parse_comments, index)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::ToString, |ctx, this, _: Value| {
			this.0.hex().to_lua(ctx)
		});

		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"r" => convert_type::<u8>(this.0.r(), ctx)?,
				"g" => convert_type::<u8>(this.0.g(), ctx)?,
				"b" => convert_type::<u8>(this.0.b(), ctx)?,
				_ => Value::Nil,
			})
		});

		/// @desc Converts the color to a hex color string
		/// @method
		/// @return {string} The converted hex color
		methods.add_method("hex", |ctx, this, _: Value| {
			this.0.hex().to_lua(ctx)
		});
	}
}

#[lua_document("TBColour", requireable = "colour")]
#[allow(unused_doc_comments)]
impl Requireable for TBColour {
	/// @desc Creates a requireable module
	/// @return {table} The colour module
	fn create_module(ctx: rlua::Context) -> rlua::Value {
		let value = ctx.create_table();

		if value.is_err() {
			return rlua::Nil;
		}

		let value = value.unwrap();

		/// @desc Creates a new colour
		/// @function
		/// @param {u32} params The u32 value to create the colour with
		/// @return {TBColour} The new timestamp
		let func = ctx.create_function(|_, params: u32| {
			let value = params;

			Ok(TBColour::new(Colour::new(value)))
		});

		value.set("new", func.unwrap()).unwrap();

		/// @desc Creates a new colour with rgb values
		/// @function
		/// @param {u8} r The red value of the color between 0 and 255
		/// @param {u8} g The green value of the color between 0 and 255
		/// @param {u8} b The blue value of the color between 0 and 255
		/// @return {TBColour} The new timestamp
		let from_rgb = ctx.create_function(|_, (r, g, b): (u8, u8, u8)| {
			Ok(TBColour::new(Colour::from_rgb(r, g, b)))
		});

		value.set("from_rgb", from_rgb.unwrap()).unwrap();

		Value::Table(value.clone())
	}
}
