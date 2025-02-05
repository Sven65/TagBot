# TBEmbed

# Methods

## set_footer_icon_url(url)

Set the footer icon URL. This only supports HTTP(S).

### Params
- url :: string | The url to set


## set_timestamp(timestamp)

Sets the timestamp of the embed

### Params
- timestamp :: [TBTimestamp](TBTimestamp.md) | The timestamp to set


## add_field(name,value,inline)

Adds a field to the embed

### Params
- name :: string | The title of the field
- value :: string | The value of the field
- inline :: bool? | Optional if the field should be inline


## set_image(url)

Sets the image for the embed

### Params
- url :: string | The URL to the image


## set_title(title)

Sets the title of the embed

### Params
- title :: String | The title to set


## set_author_icon_url(url)

Sets the authors icon url

### Params
- url :: string | The url to set


## set_description(description)

Sets the description of the embed

### Params
- description :: string | The description


## set_author_url(url)

Sets the author url

### Params
- url :: string | The Url name to set


## set_footer_text(text)

Sets the embed footers text

### Params
- text :: string | The text to set


## set_author_name(name)

Sets the name of the author

### Params
- name :: string | The name to set


## set_colour(colour)

Sets the left side colour of the embed

### Params
- colour :: [TBColour](TBColour.md) | The colour to set


## set_thumbnail(url)

Sets the thumbnail of the embed

### Params
- url :: string | The url of the thumbnail


## set_url(url)

Sets the url of the embed to direct to when clicking the title

### Params
- url :: String | The url to set


# Requireable

This module is requireable as `embed`.

## Functions

## new() -> TBEmbed

Creates a new embed



### Return Values
- :: TBEmbed | The new embed

