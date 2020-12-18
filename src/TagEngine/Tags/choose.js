class Choose {
	static randomFromArray (arr) {
		return arr[Math.floor(Math.random() * (arr.length-1 - 0 + 1)) + 0]
	}

	static execute (message, args) {
		let options
		if (args.length > 2) {
			options = args
		} else {
			options = args[0].split('|')
		}

		console.log("options", options)
		return this.randomFromArray(options)
	}
}

module.exports = Choose
