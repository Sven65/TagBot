module.exports = {
	Execute: (Args, message) => {
		try{
			if(Args.length >= 1){
				let Command = Args[0].toLowerCase();
				if(TagBot.Commands.All.indexOf(Command) > -1){

					let helpMsg = `__**${Command.capFirst()}**__\n\n`;
					helpMsg += TagBot.Commands.List[TagBot.Commands.Map[Command]][Command].Description+"\n\n";

					helpMsg += `**Usage: **\`${prefix}${Command.capFirst()}\` ${TagBot.Commands.List[TagBot.Commands.Map[Command]][Command].Usage}\n\n`;
					helpMsg += `**Cooldown: ** ${TagBot.Commands.List[TagBot.Commands.Map[Command]][Command].Cooldown.formatNumber()} seconds.`;
					if(TagBot.Commands.List[TagBot.Commands.Map[Command]][Command].hasOwnProperty("Extra")){
						for(let Extra in TagBot.Commands.List[TagBot.Commands.Map[Command]][Command].Extra){
							helpMsg += "\n";
							helpMsg += `**${Extra.replace("__", " ")}: `;
							if(Array.isArray(TagBot.Commands.List[TagBot.Commands.Map[Command]][Command].Extra[Extra])){
								helpMsg += `${TagBot.Commands.List[TagBot.Commands.Map[Command]][Command].Extra[Extra].join(', ')}`;
								helpMsg += "**";
							}
						}
					}

					let cmd = TagBot.Commands.List[TagBot.Commands.Map[Command]][Command];

					let embedPerms = false;

					if(message.channel.type !== "text"){
						embedPerms = true;
					}else{
						if(message.channel.permissionsFor(TagBot.user).hasPermission("EMBED_LINKS")){
							embedPerms = true;
						}
					}

					if(!embedPerms){
						message.channel.sendMessage(helpMsg);
					}else{
						let embed = {
							"embed": {
								"title": "",
								"type": "rich",
								"color": 0x2ead67,
								"author": {
									"name": `${Command.capFirst()}`
								},
								"description": cmd.Description,
								"fields": [
									{
										"name": "Usage",
										"value": `${prefix}${Command} ${cmd.Usage}`,
										"inline": true
									},
									{
										"name": "Cooldown",
										"value": `${cmd.Cooldown.formatNumber()} Seconds`,
										"inline": true
									}
								]
							}
						};

						for(let Extra in cmd.Extra){
							helpMsg += "\n";
							helpMsg += `**${Extra.replace("__", " ")}: `;
							if(Array.isArray(cmd.Extra[Extra])){
								embed.embed.fields.push({
									"name": Extra.replace("__", " "),
									"value": cmd.Extra[Extra].join(', '),
									"inline": true
								});
							}else{
								embed.embed.fields.push({
									"name": Extra.replace("__", " "),
									"value": cmd.Extra[Extra],
									"inline": true
								})
							}
						}

						message.channel.sendMessage('', embed).catch((e) => {
							TagBot.sendError(message, e);
						});
					}
				}
			}else{
				let msg = `Hello, ${message.author.username}. I'm ${TagBot.user.username}.\nFor a list of the commands I recognize, you can type \`£commands\`, \`${TagBot.user.username} commands\` or ${TagBot.user} commands`;
					msg += `\nFor more information and documentaion, please visit the wiki at https://github.com/Sven65/Tagbot/wiki`;
				message.channel.sendMessage(msg);
			}
		}catch(e){
			TagBot.sendError(message, e);
		}
	},
	desc: "Shows the help message",
	usage: "`[command]`",
	cooldown: 10,
	cmsg: 5
}