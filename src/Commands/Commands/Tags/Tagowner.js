const Tags = require(`${__dirname}/../../../Classes/Tags/Tags`)

class Tagowner{
	constructor(){
		this._Metadata = {
			cooldown: 10,
			description: "Shows the owner of a tag",
			usage: "`<name>`",
		}
	}

	async Execute(Args, message){
		if(Args.length >= 1){
			const tags = new Tags()

			let tagName = Args[0]

			tags.getTag(tagName.toLowerCase()).then(tag => {
				if(tag === null){
					message.channel.send(`:x: Sorry, ${message.author.username}, but that tag doesn't exist.`)
					return
				}

				message.client.fetchUser(tag.owner).then(User => {
					if(User !== null){
						message.channel.send(`The tag \`${tagName}\` is owned by ${User.username}#${User.discriminator}`)
					}else{
						message.channel.send("Couldn't find the tags owner.")
					}
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

module.exports = Tagowner