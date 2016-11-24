module.exports = {
	Execute: (Args, message) => {

		Tags.fromUser(message.author.id).then((tags) => {

			if(tags === null){
				message.channel.sendMessage(`:x: You don't have any tags ${message.author.username}`);
				return;
			}

			tags.toArray().then((tags) => {
				let tagArray = [];

				tags.map((Tag) => {
					tagArray.push(Tag.id);
				});

				let Msg = `Found ${tagArray.length.formatNumber()} tags!\n${tagArray.join(", ")}`;

				if(Msg.length >= 2000){
					let buf = new Buffer(tagArray.join("\r\n"), "utf-8");
					message.channel.sendFile(buf, "Tags.txt", `Found ${tagArray.length.formatNumber()} tags.`);
				}else{
					message.channel.sendMessage(Msg);
				}
			}).catch((e) => {
				TagBot.SendError(message, e);
			});
		}).catch((e) => {
			TagBot.SendError(message, e);
		});
	},
	Description: "Gives a list of a users tags",
	Usage: "",
	Cooldown: 60
}