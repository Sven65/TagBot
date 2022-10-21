# TBColour

 Wrapper for [`serenity::utils::Colour`]
# Attributes
- r :: u8
- g :: u8
- b :: u8
# Methods

## create_thing(my_param,second_table) -> table

creates the thing!!!!

### Params
- my_param.index :: string | The index in the table
- second_table.index :: string | The index in the  second table
- second_table.size :: string | how big it is


### Return Values
- r_table :: table | The created thing

- r_table.actual :: string | how big it actually is

## hex() -> string

Converts the color to a hex color string



### Return Values
- :: string | The converted hex color

# Requireable

This module is requireable as `colour`.

## Functions

## from_rgb(r,g,b) -> TBColour

Creates a new colour with rgb values

### Params
- r :: u8 | The red value of the color between 0 and 255
- g :: u8 | The green value of the color between 0 and 255
- b :: u8 | The blue value of the color between 0 and 255


### Return Values
- :: TBColour | The new colour

## new(params) -> TBColour

Creates a new colour

### Params
- params :: u32 | The u32 value to create the colour with


### Return Values
- :: TBColour | The new colour

