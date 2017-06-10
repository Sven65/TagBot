const Tags = require(`${__dirname}/../../../Classes/Tags/Tags`)
const fuzzy = require('fuzzy')

class Search{
	constructor(){
		this._Metadata = {
			cooldown: 60,
			description: "Searches for tags",
			usage: "`<term>`"
		}
	}

	searchTag(tagName, tags){
		return new Promise(resolve => {

			const options = {
				extract: function(el){ return el.id}
			}

			let results = fuzzy.filter(tagName, tags, options)

			let matches = results.map(el => {
				return el.original
			})

			resolve(matches)
		})
	}

	Execute(Args, message){
		if(Args.length >= 1){
			const tagClass = new Tags()

			let searchFor = Args.join(" ")

			tagClass.getAll().then(tags => {
				return tags.toArray()
			}).then(tags => {
				if(tags === null || tags === undefined){
					message.channel.send(`No tags found.`)
					return
				}

				this.searchTag(searchFor, tags).then(results => {

					if(results === null || results === undefined){
						message.channel.send(`No tags were found, ${message.author.username}`)
						return
					}

					let tagArray = results.map(result => result.id)

					let toSend = `Found ${tagArray.length.formatNumber()} tags!\n${tagArray.join(", ")}`;

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
			}).catch(e => message.client.sendError(message, e))
		}else{
			message.channel.send(`:x: Not enough arguments, ${message.author.username}.`)
		}
	}

	get Metadata(){
		return this._Metadata
	}
}

module.exports = Search