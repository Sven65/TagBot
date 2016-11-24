module.exports = {
	Execute: (Args, message) => {

		Tags.getAll().then((tags) => {
			tags.toArray().then((tags) => {

				let tagArray = [];

				tags.map((Tag) => {
					tagArray.push(Tag.id);
				});

				let buf = new Buffer(tagArray.join("\r\n"), "utf-8");
				message.channel.sendFile(buf, "Tags.txt", `Found ${tagArray.length.formatNumber()} tags!`);
			}).catch((e) => {
				TagBot.SendError(message, e);
			});
		}).catch((e) => {
			TagBot.SendError(message, e);
		});
	},
	Description: "Gives a list of all tags",
	Usage: "",
	Cooldown: 60
}