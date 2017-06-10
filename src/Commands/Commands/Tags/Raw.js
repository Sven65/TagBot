const Tags = require(`${__dirname}/../../../Classes/Tags/Tags`)

class Raw{
	constructor(){
		this._Metadata = {
			cooldown: 10,
			description: "Shows the raw content of a tag",
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

				message.channel.send(`\`${tag.content}\``)
			}).catch(e => message.client.sendError(message, e))
		}else{
			message.channel.send(`:x: Not enough arguments, ${message.author.username}.`)
		}
	}

	get Metadata(){
		return this._Metadata
	}
}

module.exports = Raw