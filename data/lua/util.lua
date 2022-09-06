local util = {
	__VERSION 		= "util 1.0.0",
	__DESCRIPTION 	= "Utilities for tagbot",
}

function util.choose(choices)
	return choices[math.random(#choices)]
end

function util.rint(min, max)
	return math.random(min, max)
end

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



