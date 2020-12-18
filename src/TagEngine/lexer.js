const TAG_TYPES = Object.freeze({
	CALL: 0,
	RAW: 1,
	TAG: 2,
	ARGUMENT: 3,
	SPACE: 4,
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

				if (this._currentNode.parent) {
					if (content[i + 1]) {
						this.goOver()
					}
				}
			} else if (c === '(') {
				if (this._currentNode.type === TAG_TYPES.RAW && !this._currentNode.parent && this._currentNode.parent.type !== TAG_TYPES.CALL) {
					this._currentNode.text += '('
				}

				if (this._currentNode.type === TAG_TYPES.CALL) {
					this.goDown(TAG_TYPES.ARGUMENT)
				}
			} else if (c === ')') {
				this.goUp()
			} else if (c === ' ') {
				/* This lets us add the spaces in the tags to the final messages properly.
				 * There is 100% a better way to do this, but I'm really lazy, and this works.
				 * If you're reading this, feel free to submit a PR if you're able to do it better!
				 *
				 * For reference of where this is actually used, look for `ast.type === Lexer.TAG_TYPES.SPACE` in index.js in this directory.
				 * 
				 */
				if (this._currentNode.type === TAG_TYPES.CALL && !this._currentNode.parent) {
					this.goDown(TAG_TYPES.SPACE)
					this.goUp()
				}

				this._currentNode.text += c
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
