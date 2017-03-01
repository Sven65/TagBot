module.exports = {
	Execute: (Args, message) => {
		if(message.author.id !== "141610251299454976"){
			return;
		}
		if(Args.length >= 1){
			let user = new User.User(message.author.id);
			let Tag = Args[0];

			Tags.getTag(Tag.toLowerCase()).then((tag) => {
				if(tag === null){
					message.channel.sendMessage(`:x: Sorry, ${message.author.username}, but that tag doesn't exist.`);
					return;
				}

				Tags.setOwner(Tag.toLowerCase(), message.author.id).then(() => {
					message.channel.sendMessage(`:white_check_mark: Stole tag \`${Tag}\`!`);
				}).catch((e) => {
					TagBot.SendError(message, e);
				});

			}).catch((e) => {
				TagBot.SendError(message, e);
			});
		}else{
			message.channel.sendMessage(`:x: Not enough arguments, ${message.author.username}.`);
		}
	},
	Description: "Steals a tag",
	Usage: "`<Name>`",
	Cooldown: 10
}