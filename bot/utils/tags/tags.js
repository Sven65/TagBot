module.exports = class Tags{
	constructor(){

	}

	getTag(name){
		return TagBot.rdb.r.table("Tags").get(name).default(null).run(TagBot.rdb.conn);
	}

	addTag(name, content, owner){
		let data = {
			id: name,
			content: content,
			owner: owner
		};
		return TagBot.rdb.r.table("Tags").insert(data).run(TagBot.rdb.conn);
	}
}