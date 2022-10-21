# TagBot lua docs

## Requireables

The TagBot lua executor comes with a couple of built-in modules, which can be used with `require`:

- `util` :: Module | A module with utils such as choose, rint and dump. See [util.lua](../../data/lua/util.lua) for more docs.
- `variables/sender` :: [TBUser](./TBUser.md) | The user that executed the tag
- `variables/sender_member` :: [TBMember?](./TBMember.md) | The discord guild member that executed the tag, if the tag was executed in a guild. Otherwise returns `nil`
- `variables/channel_id` :: [TBChannelId](./TBChannelId.md) | The ID of the channel where the tag was executed
- `variables/guild_id` :: [TBGuildId?](./TBGuildId.md) | The ID of the guild where the tag was executed, if it was executed in a guild
- `timestamp` :: [TBTimestamp](./TBTimestamp.md) | Module for working with timestamps
- `colour` :: [TBColour](./TBColour.md) | Module for working with colours
