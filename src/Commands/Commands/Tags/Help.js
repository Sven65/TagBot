const { MessageEmbed: Embed } = require('discord.js')

class Help{
	constructor(){
		this._Metadata = {
			cooldown: 10,
			description: "Shows the help message",
			usage: "`[command]`",
		}
	}

	async Execute(Args, message){
		const client = message.client
		const prefix = "£"
		if(Args.length >= 1){
			try{
				let Command = Args[0].toLowerCase()

				if(client.commands.Commands.All.indexOf(Command) > -1){

					const commandName = Command.capFirst()
					const commandData = client.commands.resolveCommand(Command).Metadata

					let helpMessage = `__**${commandName}**__\n\n`
					helpMessage += `${commandData.description}\n\n`
					helpMessage += `**Usage: **${prefix}${commandName} ${commandData.usage}\n\n`
					helpMessage += `**Cooldown: ** ${commandData.cooldown.formatNumber()}**\n\n`
					if(commandData.hasOwnProperty("extra")){
						for(const extra in commandData.extra){
							helpMessage += "\n"
							helpMessage += `**${extra.replace("__", " ")}: `
							if(Array.isArray(commandData.extra[extra])){
								helpMessage += `${commandData.extra[extra].join(', ')}`
								helpMessage += `**`
							}
						}
					}

					if(commandData.hasOwnProperty("Alias")){
						helpMessage += "\n"
						helpMessage += `**Aliases**: ${commandData.Alias.join(",")}`
					}


					const embedPerms = client.hasEmbedPerms(message)

					if(!embedPerms){
						message.channel.send(helpMessage)
					}else{
						const embed = new Embed()
						embed.setColor(0x2ead67)
							.setAuthor(commandName)
							.setDescription(commandData.description)
							.addField("Usage", `\`${prefix}${commandName}\` ${commandData.usage}`, true)
							.addField("Cooldown", `${commandData.cooldown.formatNumber()} Seconds`, true)

						if(commandData.hasOwnProperty("extra")){
							for(const extra in commandData.extra){
								if(Array.isArray(commandData.extra[extra])){
									embed.addField(extra.replace("__", " "), commandData.extra[extra].join(', '), true)
								}else{
									embed.addField(extra.replace("__", " "), commandData.extra[extra], true)
								}
							}
						}

						if(commandData.hasOwnProperty("Alias")){
							embed.addField(`Aliases`, commandData.Alias.map(alias => `\`${alias}\``).join(", "), true)
						}

						message.channel.send("", { embed: embed })
					}
				}
			}catch(e){
				message.client.sendError(message, e)
			}
		}else{
			let msg = `Hello, ${message.author.username}. I'm ${client.user.username}.\n`
			msg += `For a list of the commands I recognize, you can type \`${prefix}commands\``
			msg+= `, \`${client.user.username} commands\` or <@${client.user.id}> commands.`
			msg += `\nFor more information and documentaion, please visit the wiki at https://github.com/Sven65/Tagbot/wiki`

			message.channel.send(msg)
		}
	}

	get Metadata(){
		return this._Metadata
	}
}

module.exports = Help