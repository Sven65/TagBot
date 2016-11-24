module.exports = {
	Handle: (message) => {
		if(message.author.bot) return;

		let Args = message.content.replace(/\s\s+/g, " ").split(" ");

		let prefix = "£";		
		let prefixType = -1; // -1: None, 0: Standard, 1: Username, 2: Mention

		let command = "";

		if(Args[0].startsWith(prefix)){
			prefixType = 0;
			command = Args[0].replace(prefix, "").toLowerCase();
		}else if(Args[0].toLowerCase() === TagBot.user.username.toLowerCase()){
			prefixType = 1;
			Args.shift();
			command = Args[0].toLowerCase();
		}else if(message.isMentioned(TagBot.user)){
			prefixType = 2;
			Args.shift();
			command = Args[0].toLowerCase();
		}

		if(prefixType >= 0){
			if(TagBot.Commands.All.indexOf(command) > -1){
				try{
					let user = new User.User(message.author.id);
					user.isFirstTime(command).then((FirstTime) => {
						user.getLastExec(command).then((lastExecTime) => {
							let now = new Date().valueOf();
							if(now <= lastExecTime+TagBot.CommandHelper.resolveCooldown(command)*1000 && FirstTime){
								let time = Math.round(((lastExecTime + TagBot.CommandHelper.resolveCooldown(command) * 1000) - now) / 1000);
								message.channel.sendMessage(`You need to calm down, ${message.author.username}. :hourglass: ${time} seconds`);
							}else{
								try{
									Args.shift();
									TagBot.CommandHelper.resolveCommand(command).Execute(Args, message);
									user.setFirstTime(command, true).then(() => {
										user.setLastExec(command, now).then(() => {

										});
									})
								}catch(e){
									TagBot.SendError(message,e);
								}
							}
						}).catch((e) => {
							TagBot.SendError(message, e);
						});
					}).catch((e) => {
						TagBot.SendError(message, e);
					});
				}catch(e){
					TagBot.SendError(message, e);
				}
			}else{
				Tags.getTag(command).then((tag) => {
					if(tag !== null){
						let user = new User.User(message.author.id);
						user.isFirstTime("tagexec").then((FirstTime) => {
							user.getLastExec("tagexec").then((lastExecTime) => {
								let now = new Date().valueOf();
								if(now <= lastExecTime+TagBot.Config.TagCooldown*1000 && FirstTime){
									let time = Math.round(((lastExecTime + TagBot.Config.TagCooldown* 1000) - now) / 1000);
									message.channel.sendMessage(`You need to calm down, ${message.author.username}. :hourglass: ${time} seconds`);
								}else{
									try{
										Args.shift();
										TagBot.TagHandler.Handle(Args, message, tag);
										user.setFirstTime("tagexec", true).then(() => {
											user.setLastExec("tagexec", now).then(() => {

											});
										})
									}catch(e){
										TagBot.SendError(message,e);
									}
								}
							}).catch((e) => {
								TagBot.SendError(message, e);
							});
						}).catch((e) => {
							TagBot.SendError(message, e);
						});
					}
				}).catch((e) => {
					TagBot.SendError(message, e);
				});
			}
		}
	}
}