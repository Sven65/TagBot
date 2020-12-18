const fs = require('fs')

const Lexer = require('./lexer')

const Util = require("util")
const UnknownTagError = require('../UnknownTagError')

class TagEngine {
	static subTags = {}

	static loadTags () {
		const files = fs.readdirSync(`${__dirname}/Tags`)

		files.forEach(file => {
			if (!file.endsWith('.js')) return
			
			try {
				const tagReq = require(`${__dirname}/Tags/${file}`)

				if (tagReq.tagType === 'multiple') {
					Object.keys(tagReq.tagList).forEach(item => {
						this.subTags[item] = {
							execute: tagReq.tagList[item]
						}
					})
				} else {
					const name = file.slice(0, -3).toLowerCase()

					this.subTags[name] = tagReq
				}
			} catch(e){
				console.log(`error file: ${file}`)
				console.error(e)
				//reject(e)
			}
		})
	}


	static iterate (list, options, state) {
		if (list.length === 0) {
			return []
		}

		const iterator = list[Symbol.iterator]()
		const values = []

		try {
			for (const child of iterator) {
				values.push(this.interpret(child, options, state))
			}
		} catch (e) {
			throw e
			return []
		}

		return values
	}

	static interpret (ast, options, state = {}) {
		let output = ''

		if (ast.type === Lexer.TAG_TYPES.RAW) {
			output += ast.text

			if (ast.children.length > 0) {
				output += this.iterate(ast.children, options, state).join('')
			}
		} else if (ast.type === Lexer.TAG_TYPES.CALL) {
			const tagName = ast.text.trim()

			if (ast.children.length > 0) {
				output += this.iterate(ast.children, options, state).join('')
			}

			if (tagName === '') {
				// How did we get here?
			} else {
				const tag = this.subTags[tagName]

				if (!tag) {
					throw new UnknownTagError(tagName)
				} else {
					let funcArgs = ''
					if (ast.children[0]) {
						funcArgs = ast.children[0].text
					}
					output += tag.execute(options.message, funcArgs.split(','))
				}
			}
		} else if (ast.type === Lexer.TAG_TYPES.SPACE) {
			output += ' '
		}

		return output
	}

	static handle (args, message, tag) {
		const parsed = Lexer.lex(tag.content)

		console.log("parsed", parsed)

		let finalMessage

		try {
			finalMessage = this.interpret(parsed, {message, args})
		} catch (e) {
			message.channel.send(`Error when running tag ${tag.id}!\n${e.message}`)
			return
		}

		message.channel.send(finalMessage)
	}
}

module.exports = TagEngine
