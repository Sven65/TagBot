# Sender member

The guild member data for the tag executor, only available when tag is executed in a guild, will error in DMs.

```lua
local member = require("variables/sender_member")
```

## Fields

- id: string
- name: string
- avatar: string or nil
- banner: string or nil
- bot: boolean
- discriminator: string
- tag: string