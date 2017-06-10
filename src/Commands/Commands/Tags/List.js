const Tags = require(`${__dirname}/../../../Classes/Tags/Tags`)

class List{
	constructor(){
		this._Metadata = {
			cooldown: 60,
			description: "Gives a list of a users tags",
			usage: ""
		}
	}

	Execute(Args, message){
		const tagClass = new Tags()

		tagClass.fromUser(message.author.id).then(tags => {
			if(tags === null){
				message.channel.send(`:x: You don't have any tags ${message.author.username}`)
				return
			}

			return tags.toArray()
		}).then(tags => {
			if(tags === undefined || tags === null){
				return
			}

			const tagArray = tags.map(tag => tag.id)

			let toSend = `Found ${tagArray.length.formatNumber()} tags!\n${tagArray.join(", ")}`

			if(toSend.length >= 2000){
				let buffer = new Buffer(tagArray.join("\r\n"), "utf-8")
				message.channel.send(`Found ${tagArray.length.formatNumber()} tags!`, {
					files: [
						{
							attachment: buffer,
							name: `Tags.txt`
						}
					]
				})
			}else{
				message.channel.send(toSend)
			}
		}).catch(e => message.client.sendError(message, e))
	}

	get Metadata(){
		return this._Metadata
	}
}

module.exports = List