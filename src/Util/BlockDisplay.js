class BlockDisplay{

	/** Constructs a new BlockDisplay
	 * @constructs BlockDisplay
	 * @memberof Util
	 * @param {?string} title - The title to use for the display (Top Line)
	 * @param {?string} footer - The footer to use for the display (Bottom Line)
	 * @param {?string} lang - The language to use for the display (Codeblock Language)
	 * @author Mackan
	 */
	constructor(title="", footer="", lang=""){
		this._lines = []
		this._lang = lang
		this._title = title
		this._footer = footer
	}

	/**
	 * Sets the code language of the BlockDisplay
	 * @type {string}
	 * @memberof BlockDisplay
	 */
	set lang(lang){
		this._lang = lang
	}

	/**
	 * Sets the title of the BlockDisplay
	 * @type {string}
	 * @memberof BlockDisplay
	 */
	set title(title){
		this._title = title
	}

	/**
	 * Sets the footer of the BlockDisplay
	 * @type {string}
	 * @memberof BlockDisplay
	 */
	set footer(footer){
		this._footer = footer
	}

	/** Adds a line to the block display
	 * @function
	 * @memberof BlockDisplay
	 * @param {string|Array.<string>} lines - The line or lines to add
	 * @author Mackan
	 */
	push(lines){
		if(Array.isArray(lines)){
			lines.map(line => this._lines.push(line))
		}else{
			this._lines.push(lines)
		}
	}

	/** Returns the BlockDisplay as a codeblock string
	 * @function
	 * @memberof BlockDisplay
	 * @returns {string}
	 * @author Mackan
	 */
	toString(){
		let block = "```"+`${this._lang!==""?this._lang:''}\n`
		if(this._title !== "" && this._title !== null){
			block += this._title+"\n"
		}


		this._lines.map(line => block += `${line}\n`)

		if(this._footer !== "" && this._footer !== null){
			block += this._footer
		}
		block += "```"

		return block
	}
}

module.exports = BlockDisplay