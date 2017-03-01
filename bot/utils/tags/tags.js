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

	editTag(name, content){
		return TagBot.rdb.r.table("Tags").get(name).update({content: content}).run(TagBot.rdb.conn);
	}

	deleteTag(name){
		return TagBot.rdb.r.table("Tags").get(name).delete().run(TagBot.rdb.conn);
	}

	getAll(){
		return TagBot.rdb.r.table("Tags").run(TagBot.rdb.conn);
	}

	fromUser(user){
		return TagBot.rdb.r.table("Tags").filter(function(tag){
			return tag("owner").eq(user)
		}).default(null).run(TagBot.rdb.conn);
	}

	setOwner(tag, owner){
		return TagBot.rdb.r.table("Tags").get(tag).update({owner: owner}).run(TagBot.rdb.conn);
	}
}