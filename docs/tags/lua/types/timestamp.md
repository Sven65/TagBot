# Timestamp

A simple timestamp

## Functions

- [`__tostring`](#tostring)
- [`format`](#formatfmt)
- [`d_format`](#dformatspecifier)

### `__tostring`

When calling `tostring(Timestamp)`, it'll be formatted as an RFC 3339 date string such as 2016-04-30T11:18:25.796Z.

### `format(fmt)`

Formats the timestamp into a string specified by the `fmt` argument, replacing the following:

| Tag | Description |
|-----|-------------|
|%%   | A literal %|
|%a   | Locale's abbreviated weekday name (e.g., Sun)|
|%A   | Locale's full weekday name (e.g., Sunday)|
|%b   | Locale's abbreviated month name (e.g., Jan)|
|%B   | Locale's full month name (e.g., January)|
|%d   | Day of month (e.g., 01)|
|%D   | Date; same as %m/%d/%y|
|%H   | Hour (00..23)|
|%m   | Month (01..12)|
|%M   | Minute (00..59)|
|%S   | Second (00..60)|
|%T   | Time; same as %H:%M:%S|
|%y   | Last two digits of year (00..99)|
|%Y   | Year |


### `d_format(specifier)`

Formats the timestamp into a Discord timestamp tag, i.e `<t:timestamp>`, where the specifier is one of `t`, `T`, `d`, `D`, `f`, `F`, `R` or an empty string.

|Style|Input|Output (12-hour clock)|Output (24-hour clock)
|--|--|--|--
|Default|`<t:1543392060>`|November 28, 2018 9:01 AM|28 November 2018 09:01
|Short Time|`<t:1543392060:t>`|9:01 AM|09:01
|Long Time|`<t:1543392060:T>`|9:01:00 AM|09:01:00
|Short Date|`<t:1543392060:d>`|11/28/2018|28/11/2018
|Long Date|`<t:1543392060:D>`|November 28, 2018|28 November 2018
|Short Date/Time|`<t:1543392060:f>`|November 28, 2018 9:01 AM|28 November 2018 09:01
|Long Date/Time|`<t:1543392060:F>`|Wednesday, November 28, 2018 9:01 AM|Wednesday, 28 November 2018 09:01
|Relative Time|`<t:1543392060:R>`|3 years ago|3 years ago

## Creating a new timestamp

```lua
local Timestamp = user_require('timestamp')
local time = Timestamp.new{secs = 1662796089}
print(tostring(time))
```