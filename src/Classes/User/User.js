class User{
	/** Creates a new user
	 * @constructs User
	 * @param {string} id - The ID of the user
	 * @author Mackan
	 */
	constructor(id){
		this._id = id
	}

	/**
	 * Gets the amount of tags that the user has
	 * @function
	 * @returns {Promise.<number>}
	 * @author Mackan
	 */
	async getTagCount(){
		return await ReDB.r.table("Tags").filter({
			Owner: this._id,
		}).count().default(0).run(ReDB.conn)
	}


	/** Checks if the user is ignored
	 * @function
	 * @returns {Promise.<boolean>}
	 * @author Mackan
	 */
	async isIgnored(){
		return await ReDB.r.table('ignore').filter({ type: "user", id: this._id }).count().gt(0).run(ReDB.conn)
	}

	/** Unignores the user
	 * @function
	 * @returns {Promise.<Cursor>}
	 * @author Mackan
	 */
	async unignore(){
		return await ReDB.r.table('ignore').filter({ type: 'user', id: this._id }).delete().run(ReDB.conn)
	}

	/** Ignores the user
	 * @function
	 * @returns {Promise.<Cursor>}
	 * @author Mackan
	 */
	async ignore(){
		return await ReDB.r.table('ignore').insert({ type: 'user', id: this._id }).run(ReDB.conn)
	}
}

module.exports = User