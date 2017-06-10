require(`${__dirname}/../Util/Extenders.js`)
const User = require(`${__dirname}/../Classes/User/User`)
const Tags = require(`${__dirname}/../Classes/Tags/Tags`)

const TH = require(`${__dirname}/Tags/Tags`)

const tagHandler = new TH()

module.exports = (message) => {

	if(message.author.bot) return

	const client = message.client
	const tagClass = new Tags()
	

	if(message.author.id !== "141610251299454976") return // FOR TEST ONLY

	if(message.channel.type === "text"){
		if(message.guild.id === "172382467385196544" && message.channel.id === "172382467385196544"){
			if(message.author.id !== client.Config.owner && message.author.id !== "120627061214806016" && message.author.id !== "158049329150427136"){
				return
			}
		}
	}

	let Args = message.content.replace(/\s\s+/g, " ").split(" ")

	const prefix = "£"
	let prefixType = -1 // -1: None, 0: Standard, 1: Username, 2: Mention
	let command = ""

	if(Args[0].startsWith(prefix)){
		prefixType = 0
	}else if(Args[0].toLowerCase() === client.user.username.toLowerCase()){
		prefixType = 1
		Args.shift()
	}else if(message.isMentioned(client.user)){
		prefixType = 2
		Args.shift()
	}

	if(Args[0] === undefined){
		message.channel.send(`What do you want, ${message.author.username}?`)
		return
	}

	command = Args[0].replace(prefix, "").toLowerCase()

	if(prefixType >= 0){
		//Args.shift()
		if(command === "eval"){
			if(message.author.id === client.Config.Owner){
				try{
					let start = new Date().getTime();
					let msg = "";
					if(Args[0] === "-c"){
						let code = Args.splice(1, Args.length).join(" ");
						msg += "```js\n"+code+"```\n";
						msg += "```js\n"+eval(code)+"```";
					}else{
						let code = Args.join(" ");
						msg += "```js\n"+eval(code)+"```";
					}

					let end = new Date().getTime();
					let time = end - start;

					message.channel.send("Time taken: "+(time/1000)+" seconds\n"+msg);
				}catch(e){
					message.channel.send("```js\n"+e+"```");
				}
			}
		}else{
			if(client.commands.Commands.All.indexOf(command) > -1){
				try{
					const user = new User(message.author.id)

					Promise.all([user.isFirstTime(command), user.getLastExec(command)]).then(values => {
						const isFirstTime = values[0]
						const lastExec = values[1]

						let now = new Date().valueOf()
						if(now <= lastExec+client.commands.resolveCooldown(command)*1000 && isFirstTime){
							let time = Math.round(((lastExec + client.commands.resolveCooldown(command) * 1000) - now) / 1000)
							message.channel.send(`You need to calm down, ${message.author.username}. :hourglass: ${time} seconds`)
						}else{
							try{
								Args.shift()
								client.commands.resolveCommand(command).Execute(Args, message)
								user.setFirstTime(command, true).then(() => {
									return user.setLastExec(command, now)
								}).catch(e => message.client.sendError(message, e))
							}catch(e){
								message.client.sendError(message, e)
							}
						}
					}).catch(e => message.client.sendError(message, e))
				}catch(e){
					message.client.sendError(message, e)
				}
			}else{
				tagClass.getTag(command).then(tag => {
					if(tag === undefined || tag === null){
						return
					}

					const user = new User(message.author.id)

					Promise.all([user.isFirstTime("tagexec"), user.getLastExec("tagexec")]).then(values => {
						const isFirstTime = values[0]
						const lastExec = values[1]

						let now = new Date().valueOf()
						if(now <= lastExec+client.Config.tagCooldown*1000 && isFirstTime){
							let time = Math.round(((lastExec + client.Config.tagCooldown * 1000) - now) / 1000)
							message.channel.send(`You need to calm down, ${message.author.username}. :hourglass: ${time} seconds`)
						}else{
							try{
								Args.shift()
								tagHandler.Handle(Args, message, tag)
								user.setFirstTime("tagexec", true).then(() => {
									return user.setLastExec("tagexec", now)
								}).catch(e => message.client.sendError(message, e))
							}catch(e){
								message.client.sendError(message, e)
							}
						}
					}).catch(e => message.client.sendError(message, e))
				}).catch(e => message.client.sendError(message, e))
			}
		}
	}
}