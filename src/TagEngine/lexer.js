const TAG_TYPES = Object.freeze({
	CALL: 0,
	RAW: 1,
	TAG: 2,
	ARGUMENT: 3,
})

class Lexer {
	static _currentNode = null

	static createNode (parent = null, type = null) {
		return {
			parent,
			type,
			value: '',
			children: [],
			text: '',
			args: [],
		}
	}

	static goUp () {
		this._currentNode = this._currentNode.parent
	}

	static goDown (type) {
		const node =  this.createNode(this._currentNode, type)
		this._currentNode.children.push(node)

		this._currentNode = node
	}

	static goOver (type) {
		const node =  this.createNode(this._currentNode, type)

		console.log("current node", this._currentNode)

		this._currentNode.parent.children.push(node)

		this._currentNode = node
	}

	static startSubTag () {

	}

	static endSubTag () {

	}

	/**
	 * Lexes the given content into an executable AST
	 * @param {string} content - The content to lex
	 * @function
	 * @returns {Array.<LexNode>}
	 * @author Mackan
	 */
	static lex (content) {
		this._currentNode = this.createNode(null, content[0] === '{' ? TAG_TYPES.CALL : TAG_TYPES.RAW)

		for (let i = 0; i < content.length; i++) {
			const c = content[i]

			if (this._currentNode.type === null) {
				this._currentNode.type = content[i - 1] === '{' ? TAG_TYPES.CALL : TAG_TYPES.RAW
			}

			if (c === '{') {
				this.goDown(TAG_TYPES.CALL)
			} else if (c === '}') {
				this.goUp()
				// if (content[i + 1]) {
				// 	this.goOver()
				// }
			} else if (c === '(') {
				console.group("LEFT PAREN")

				if (this._currentNode.type === TAG_TYPES.CALL) {
					this.goDown(TAG_TYPES.ARGUMENT)
				}

				console.log("THIS", this._currentNode)
				if (this._currentNode.type === TAG_TYPES.RAW && !this._currentNode.parent && this._currentNode.parent.type !== TAG_TYPES.CALL) {
					this._currentNode.text += '('
				}
				console.groupEnd()


			} else if (c === ')') {
				this.goUp()
			} else {
				this._currentNode.text += c
			}
		}

		while (this._currentNode.parent !== null) {
			this.goUp()
		}

		return this._currentNode
	}
}

module.exports = Lexer
module.exports.TAG_TYPES = TAG_TYPES
