class UnknownTagError extends Error {
	constructor (tagName) {
		super(`Unknown subtag: ${tagName}`)
		this.name = 'UnknownTagError'
	}
}

module.exports = UnknownTagError
