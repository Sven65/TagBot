module.exports = class Tags{
	constructor(){

	}

	getTag(name){
		return TagBot.rdb.r.table("Tags").get(name).default(null).run(TagBot.rdb.conn);
	}
}