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

				if(tag.owner !== message.author.id){
					message.channel.sendMessage(`:x: You don't own this tag, ${message.author.username}.`);
					return;
				}

				Tags.deleteTag(Tag.toLowerCase()).then(() => {
					message.channel.sendMessage(`:white_check_mark: Deleted tag \`${Tag}\`!`);
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
	Description: "Deletes a tag",
	Usage: "`<Name>`",
	Cooldown: 10
}