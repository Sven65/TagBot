const Tags = require(`${__dirname}/../../../Classes/Tags/Tags`)

class Delete{
	constructor(){
		this._Metadata = {
			cooldown: 10,
			description: "Deletes a tag",
			usage: "`<name>`"
		}
	}

	Execute(Args, message){
		if(Args.length >= 1){
			const tags = new Tags()

			let tagName = Args[0]

			tags.getTag(tagName.toLowerCase()).then(tag => {
				if(tag === null){
					message.channel.send(`:x: Sorry, ${message.author.username}, but that tag doesn't exist.`)
					return
				}

				if(tag.owner !== message.author.id){
					message.channel.send(`:x: You don't own this tag, ${message.author.username}.`)
					return
				}

				tags.deleteTag(tagName.toLowerCase()).then(() => {
					message.channel.send(`:white_check_mark: Deleted tag \`${tagName}\`!`)
				}).catch(e => message.client.sendError(message, e))
			}).catch(e => message.client.sendError(message, e))
		}else{
			message.channel.send(`:x: Not enough arguments, ${message.author.username}.`)
		}
	}

	get Metadata(){
		return this._Metadata
	}
}

module.exports = Delete