const BlockDisplay = require(`${__dirname}/../../../Util/BlockDisplay`)

class Commands{
	constructor(){
		this._Metadata = {
			cooldown: 10,
			description: "Shows commands",
			usage: ""
		}
	}

	Execute(Args, message){
		const client = message.client

		let commandMap = {}

		let textBlock = new BlockDisplay(null, null, "ini")

		client.commands.Commands.All.map(commandName => {
			const Group = client.commands.Commands.Map[commandName]
			if(commandMap[Group] === undefined){
				commandMap[Group] = []
			}

			const Commands = client.commands.Commands.List[Group]

			for(const command in Commands){
				if(commandMap[Group].indexOf(command) === -1){
					if(!Commands[command].Metadata.hasOwnProperty("unlisted")){
						commandMap[Group].push(command)
					}
				}
			}
		})

		Object.keys(commandMap).map(commandGroup => {
			const commandsInGroup = commandMap[commandGroup].join(`, `)
			textBlock.push(`[${commandGroup}]\n${commandsInGroup}`)
		})

		message.channel.send(textBlock.toString())
	}
	
	get Metadata(){
		return this._Metadata
	}
}

module.exports = Commands