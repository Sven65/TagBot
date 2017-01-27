module.exports = {
	Execute: (Args, message) => {
		if(Args.length >= 2){
			let user = new User.User(message.author.id);
			let Tag = Args[0];
			let Value = Args.splice(1, Args.length).join(" ");

			Tags.getTag(Tag.toLowerCase()).then((tag) => {
				if(tag === null){
					message.channel.sendMessage(`:x: Sorry, ${message.author.username}, but that tag doesn't exist.`);
					return;
				}

				if(tag.owner !== message.author.id){
					message.channel.sendMessage(`:x: You don't own this tag, ${message.author.username}.`);
					return;
				}

				if(Value.length < 3){
					message.channel.sendMessage(`:x: Your tag needs to have atleast 3 characters of content, ${message.author.username}.`);
					return;
				}

				Tags.editTag(Tag.toLowerCase(), Value).then(() => {
					message.channel.sendMessage(`:white_check_mark: Edited tag \`${Tag}\`!`);
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
	Description: "Edits a tag",
	Usage: "`<Name>`, `<Content>`",
	Cooldown: 10
}