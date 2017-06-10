const Tags = require(`${__dirname}/../../../Classes/Tags/Tags`)

class Stealtag{
	constructor(){
		this._Metadata = {
			cooldown: 10,
			description: "Steals a tag",
			usage: "`<name>`"
		}
	}

	Execute(Args, message){
		if(message.author.id !== "141610251299454976"){
			return
		}

		if(Args.length >= 1){
			const tags = new Tags()

			let tagName = Args[0]

			tags.getTag(tagName.toLowerCase()).then(tag => {
				if(tag === null){
					message.channel.send(`:x: Sorry, ${message.author.username}, but that tag doesn't exist.`)
					return
				}

				tags.setOwner(tagName.toLowerCase(), message.author.id).then(() => {
					message.channel.send(`:white_check_mark: Stole tag \`${Tag}\`!`)
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

module.exports = Stealtag