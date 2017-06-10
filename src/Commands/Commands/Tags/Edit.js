const Tags = require(`${__dirname}/../../../Classes/Tags/Tags`)

class Edit{
	constructor(){
		this._Metadata = {
			cooldown: 10,
			description: "Edits a tag",
			usage: "`<name>`, `<content>`"
		}
	}

	Execute(Args, message){
		if(Args.length >= 2){
			const tags = new Tags()

			let tagName = Args[0]

			let contents = Args.splice(1, Args.length).join(" ")

			tags.getTag(tagName.toLowerCase()).then(tag => {
				if(tag === null){
					message.channel.send(`:x: Sorry, ${message.author.username}, but that tag doesn't exist.`)
					return
				}

				if(tag.owner !== message.author.id){
					message.channel.send(`:x: You don't own this tag, ${message.author.username}.`)
					return
				}

				if(Value.length < 3){
					message.channel.send(`:x: Your tag needs to have atleast 3 characters of content, ${message.author.username}.`)
					return
				}

				tags.editTag(tagName.toLowerCase(), content).then(() => {
					message.channel.send(`:white_check_mark: Edited tag \`${tagName}\`!`)
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

module.exports = Edit