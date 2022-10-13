--- Utility functions

local util = {
	__VERSION 		= "util 1.0.0",
	__DESCRIPTION 	= "Utilities for tagbot",
}

--- Chooses an item from a table
-- @param choices The choices to choose from
-- @return A value from choices
function util.choose(choices)
	return choices[math.random(#choices)]
end

--- Generates a number between min and max (inclusive)
-- @param min The min number
-- @param max The max number
-- @return A number
function util.rint(min, max)
	return math.random(min, max)
end

--- Dumps a table to string
-- @param o The table to dump
-- @return The dumped table as a string
function util.dump(o)
	if type(o) == 'table' then
		local s = '{ '
		for k,v in pairs(o) do
			if type(k) ~= 'number' then k = '"'..k..'"' end
			s = s .. '['..k..'] = ' .. util.dump(v) .. ','
		end
		return s .. '} '
	else
		return tostring(o)
	end
end

return util



