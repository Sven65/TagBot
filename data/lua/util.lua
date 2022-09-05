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

return util



