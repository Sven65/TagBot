class User{
	constructor(id){
		this._id = id
	}

	getLastExec(command){
		const now = new Date().valueOf()
		return ReDB.r.table("Cooldowns").get(this._id)(command).default(now).run(ReDB.conn)
	}

	setLastExec(command, exec){
		let data = {id: this._id};
		data[command] = exec;
		return ReDB.r.table("Cooldowns").insert(data, {conflict: "update"}).run(ReDB.conn)
	}

	isFirstTime(command){
		return ReDB.r.table('FirstTime').get(this._id)(command).default(false).run(ReDB.conn)
	}

	setFirstTime(command, time){
		let data = {id: this._id};
		data[command] = time
		return ReDB.r.table('FirstTime').insert(data, {conflict: "update"}).run(ReDB.conn)
	}

	getTagCount(){
		let id = this._id
		return ReDB.r.table("Tags").filter(function(tag){
			return tag("Owner").eq(id)
		}).count().default(0).run(ReDB.conn)
	}
}

module.exports = User