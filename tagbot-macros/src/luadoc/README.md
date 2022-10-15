# Lua Doc Generator

Macro for generating LuaDoc files.

## Todo:

- [ ] Doc generation for Requireable (user_require)
- [ ] Doc comments for errors
- [ ] Document generation for TBGuildWelcomeChannelEmoji index
- [ ] Parsing of comments inside match statements of index
  - Should be treated as a desc of the attribute

## Usage

The macro is used with `#[lua_document("name", attribs)]`, where "name" is the unique name of the struct being documented and attribs is one or more attriobutes.

## Attributes

### class

Required in the document, cannot be used with other attributes.

Generates the entry point of the document.

#### Usage

```rs
#[lua_document("TBChannelCategory", class)]
pub struct TBChannelCategory(pub ChannelCategory);
```

### index

Will generate documentation about indexed parameters in `MetaMethod::Index`.

#### Usage

```rs
impl UserData for TBChannelCategory {
	#[lua_document("TBChannelCategory", index)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
		methods.add_meta_method(MetaMethod::Index, |ctx, this, value: String| {
			Ok(match value.as_str() {
				"id" => convert_constructable2::<TBChannelId, _, SerenityContext>(this.0.id, this.1.clone(), ctx)?,
				...,
				&_ => Value::Nil,
			})
		})
	}
}
```

### parse_comments

Will parse comments for luadoc-ish comments.

Currently implements `@method`, `@param`, `@return` and `@doc`.

#### Usage

```rs
impl UserData for TBTimestamp {
	#[lua_document("TBTimestamp", parse_comments)]
	#[allow(unused_doc_comments)]
	fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {

		/// @desc Formats the timestamp with with the specified format string.
		/// @method
		/// @param {string} value The [format string](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) to use when formatting
		/// @return {string} Formatted timestamp
		methods.add_method("format", |ctx, this, value: String| {
			let time = Utc.timestamp(this.unix_timestamp(), 0);

			let formatted = time.format(&value);

			Ok(formatted.to_string().to_lua(ctx))
		});
	}
}
```
