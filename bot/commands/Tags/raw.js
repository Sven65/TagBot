module.exports = {
	Execute: (Args, message) => {
		if(Args.length >= 1){
			let user = new User.User(message.author.id);
			let Tag = Args[0];

			Tags.getTag(Tag.toLowerCase()).then((tag) => {
				if(tag === null){
					message.channel.sendMessage(`:x: Sorry, ${message.author.username}, but that tag doesn't exist.`);
					return;
				}

				message.channel.sendMessage(`\`${tag.content}\``);

			}).catch((e) => {
				TagBot.SendError(message, e);
			});
		}else{
			message.channel.sendMessage(`:x: Not enough arguments, ${message.author.username}.`);
		}
	},
	Description: "Shows the raw content of a tag",
	Usage: "`<Name>`",
	Cooldown: 10
}