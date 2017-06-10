class Tags{
	constructor(){

	}

	getTag(name){
		return ReDB.r.table("Tags").get(name).default(null).run(ReDB.conn)
	}

	addTag(name, content, owner){
		let data = {
			id: name,
			content: content,
			owner: owner
		}
		return ReDB.r.table("Tags").insert(data).run(ReDB.conn)
	}

	editTag(name, content){
		return ReDB.r.table("Tags").get(name).update({content: content}).run(ReDB.conn)
	}

	deleteTag(name){
		return ReDB.r.table("Tags").get(name).delete().run(ReDB.conn)
	}

	getAll(){
		return ReDB.r.table("Tags").run(ReDB.conn)
	}

	fromUser(user){
		return ReDB.r.table("Tags").filter(function(tag){
			return tag("owner").eq(user)
		}).default(null).run(ReDB.conn)
	}

	setOwner(tag, owner){
		return ReDB.r.table("Tags").get(tag).update({owner: owner}).run(ReDB.conn)
	}
}

module.exports = Tags