const User = require(`${__dirname}/../../../Classes/User/User`)
const Tags = require(`${__dirname}/../../../Classes/Tags/Tags`)

class Add{
	constructor(){
		this._Metadata = {
			cooldown: 10,
			description: "Adds a tag",
			usage: "`<name>`, `<contents>`"
		}
	}

	Execute(Args, message){
		console.log(Args)
		if(Args.length >= 2){
			const user = new User(message.author.id)
			const tags = new Tags()

			user.getTagCount().then(count => {
				if(count >= 500){
					message.channel.send(`:x: Sorry, ${message.author.username}, but you can only have 500 tags.`)
					return
				}

				let tagName = Args[0]
				let tagContent = Args.splice(1, Args.length).join(" ")

				if(message.client.commands.Commands.All.indexOf(tagName.toLowerCase()) > -1){
					message.channel.send(`:x: You can't override commands, ${message.author.username}.`)
					return
				}

				tags.getTag(tagName.toLowerCase()).then(tag => {
					if(tag !== null){
						message.channel.send(`:x: Sorry, ${message.author.username}, but that tag already exists.`)
						return
					}

					if(tagContent.length < 3){
						message.channel.send(`:x: Your tag needs to have atleast 3 characters of content, ${message.author.username}.`)
						return
					}

					tags.addTag(tagName.toLowerCase(), tagContent, message.author.id).then(() => {
						message.channel.send(`:white_check_mark: Added tag \`${tagName}\`!`)
					}).catch(e => message.client.sendError(message, e))
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

module.exports = Add