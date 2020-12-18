const Tags = require(`${__dirname}/../../../Classes/Tags/Tags`)

class Glist{
	constructor(){
		this._Metadata = {
			cooldown: 60,
			description: "Gives a list of all tags",
			usage: "",
		}
	}

	async Execute(Args, message){
		const tagClass = new Tags()

		tagClass.getAll().then(tags => {
			return tags.toArray()
		}).then(tags => {
			const tagArray = tags.map(tag => tag.id)

			let buffer = new Buffer(tagArray.join("\r\n"), "utf-8")

			message.channel.send(`Found ${tagArray.length.formatNumber()} tags!`, {
				files: [
					{
						attachment: buffer,
						name: `Tags.txt`,
					},
				],
			})
		}).catch(e => message.client.sendError(message, e))
	}

	get Metadata(){
		return this._Metadata
	}
}

module.exports = Glist