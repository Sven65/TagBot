module.exports = class User{
	constructor(id){
		this.id = id;
	}

	getLastExec(command){
		let now = new Date().valueOf();
		return TagBot.rdb.r.table("Cooldowns").get(this.id)(command).default(now).run(TagBot.rdb.conn);
	}

	setLastExec(command, exec){
		let data = {id: this.id};
		data[command] = exec;
		return TagBot.rdb.r.table("Cooldowns").insert(data, {conflict: "update"}).run(TagBot.rdb.conn);
	}

	isFirstTime(command){
		return TagBot.rdb.r.table('FirstTime').get(this.id)(command).default(false).run(TagBot.rdb.conn);
	}

	setFirstTime(command, time){
		let data = {id: this.id};
		data[command] = time;
		return TagBot.rdb.r.table('FirstTime').insert(data, {conflict: "update"}).run(TagBot.rdb.conn);
	}
}