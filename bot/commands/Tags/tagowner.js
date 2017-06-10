module.exports = {
	Execute: (Args, message) => {
		if(Args.length >= 1){
			let Tag = Args[0];

			Tags.getTag(Tag.toLowerCase()).then((tag) => {
				if(tag === null){
					message.channel.sendMessage(`:x: Sorry, ${message.author.username}, but that tag doesn't exist.`);
					return;
				}

				TagBot.fetchUser(tag.owner).then(User => {
					if(User !== null){
						message.channel.sendMessage(`The tag \`${Tag}\` is owned by ${User.username}#${User.discriminator}`);
					}else{
						message.channel.sendMessage("Couldn't find the tags owner.");
					}
				})

				

			}).catch((e) => {
				TagBot.SendError(message, e);
			});
		}else{
			message.channel.sendMessage(`:x: Not enough arguments, ${message.author.username}.`);
		}
	},
	Description: "Shows who owns a tag",
	Usage: "`<tag>`",
	Cooldown: 10
}