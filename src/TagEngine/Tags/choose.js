class Choose {
	static randomFromArray (arr) {
		return arr[Math.floor(Math.random() * (arr.length-1 - 0 + 1)) + 0]
	}

	static execute (message, args) {
		console.log("args", args)
		return this.randomFromArray(args.split("|"))
	}
}

module.exports = Choose
