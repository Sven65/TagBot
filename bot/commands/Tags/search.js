const Fuse = require("fuse.js");

module.exports = {
	Execute: (Args, message) => {

		let searchFor = Args.join(" ");

		Tags.getAll().then((tags) => {
			tags.toArray().then(tags => {
				if(tags === null || tags === undefined){
					message.channel.sendMessage(`No tags found.`);
					return;
				}

				const options = {
					shouldSort: true,
					threshold: 0.6,
					location: 0,
					distance: 100,
					maxPatternLength: 32,
					minMatchCharLength: 1,
					keys: [
						"id"
					]
				};
				const fuse = new Fuse(tags, options);
				const result = fuse.search(searchFor);
				
				let items = [];

				result.map((a, b) => {
					items.push(a.id);
				});

				let Msg = `Found ${items.length.formatNumber()} tags!\n${items.join(", ")}`;

				if(Msg.length >= 2000){
					let buf = new Buffer(items.join("\r\n"), "utf-8");
					message.channel.sendFile(buf, "Tags.txt", `Found ${items.length.formatNumber()} tags.`);
				}else{
					if(items.length <= 0){
						Msg = "No tags found.";
					}
					message.channel.sendMessage(Msg);
				}
			}).catch((e) => {
				TagBot.SendError(message, e);
			});
		}).catch((e) => {
			TagBot.SendError(message, e);
		});
	},
	Description: "Searches for tags",
	Usage: "<term>",
	Cooldown: 60
}