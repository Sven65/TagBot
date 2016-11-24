module.exports = {
	Execute: (Args, message) => {
		if(Args.length >= 2){
			let user = new User.User(message.author.id);
			user.tagCount().then((count) => {
				if(count >= 500){
					message.channel.sendMessage(`:x: Sorry, ${message.author.username}, but you can only have 500 tags.`);
					return;
				}

				let Tag = Args[0];
				let Value = Args.splice(1, Args.length).join(" ");

				if(TagBot.Commands.All.indexOf(Tag.toLowerCase()) > -1){
					message.channel.sendMessage(`:x: You can't override commands, ${message.author.username}.`);
					return;
				}

				Tags.getTag(Tag.toLowerCase()).then((tag) => {
					if(tag !== null){
						message.channel.sendMessage(`:x: Sorry, ${message.author.username}, but that tag already exists.`);
						return;
					}

					if(Value.length < 3){
						message.channel.sendMessage(`:x: Your tag needs to have atleast 3 characters of content, ${message.author.username}.`);
						return;
					}

					Tags.addTag(Tag.toLowerCase(), Value, message.author.id).then(() => {
						message.channel.sendMessage(`:white_check_mark: Added tag \`${Tag}\`!`);
					}).catch((e) => {
						TagBot.SendError(message, e);
					});

				}).catch((e) => {
					TagBot.SendError(message, e);
				});

			}).catch((e) => {
				TagBot.SendError(message, e);
			})
		}else{
			message.channel.sendMessage(`:x: Not enough arguments, ${message.author.username}.`);
		}
	},
	Description: "Adds a tag",
	Usage: "`<Name>`, `<Content>`",
	Cooldown: 10
}