module.exports = {
	Execute: (Args, message) => {
		try{
			let x = {};

			let m = "```ini\n";
			TagBot.Commands.All.map((a) => {
				let Group = TagBot.Commands.Map[a];
				if(x[Group] === undefined){
					x[Group] = [];
				}

				let Commands = TagBot.Commands.List[Group];

				for(Command in Commands){
					if(x[Group].indexOf(Command) === -1){
						if(!Commands[Command].hasOwnProperty("unlisted")){
							x[Group].push(Command);
						}
					}
				}
			});
			Object.keys(x).map((a) => {
				let b = x[a].join(", ");
				m += `[${a}]\n${b}\n`;
			});

			m += "```";
			
			message.channel.sendMessage(m).catch((e) => {
				console.dir(e);
			});
		}catch(e){
			TagBot.SendError(message, e);
		}
	},
	desc: "Shows commands",
	usage: "",
	cooldown: 10,
	cmsg: 5
}