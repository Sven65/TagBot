let generateTemplateString = (() => {
	let cache = {};
	function generateTemplate(template){
		let fn = cache[template];
		if(!fn){
			// Replace ${expressions} (etc) with ${map.expressions}.
			let sanitized = template.replace(/\$\{([\s]*[^;\s]+[\s]*)\}/g, (_, match) => {
				return `\$\{map.${match.trim()}\}`;
			}).replace(/(\$\{(?!map\.)[^}]+\})/g, '');
			fn = Function('map', `return \`${sanitized}\``);
		}
		return fn;
	};
	return generateTemplate;
})();

module.exports = {
	Handle: (Args, Message, Tag) => {
		let Msg = Tag.content.format(Args);

		let TagObject = {
			// Sender variables, 's' = Sender

			sname: Message.author.username,
			sid: Message.author.id,
			sdiscrim: Message.author.discriminator,
			sstatus: Message.author.presence.status,
			sgame: Message.author.presence.game||"Unknown",
			sbot: Message.author.bot,
			sender: Message.author,

			// Member variables of the sender

			sjoined: Message.member!==null?Message.member.joinedAt.toUTCString():"Unknown",
			snick: Message.author.username,

			// Message variables, 'm' = Message

			mtime: Message.createdAt.toUTCString(),
			mid: Message.id,

			// Channel variables, 'chan' = Channel

			chanid: Message.channel.id,
			chantype: Message.channel.type,
			chancreated: Message.channel.createdAt.toUTCString(),
			channame: Message.channel.name||"Unknown",
			chantopic: Message.channel.topic||"Unknown",

			// Server variables, 'server' = Server

			serverregion: "Unknown",
			servername: "Unknown",
			serverid: "Unknown",
			servermembs: "Unknown",
			serverchans: "Unknown",
			serverdefchan: "Unknown",
			servercreated: "Unknown",
			serververification: "Unknown",

			// Server owner variables

			serverowner: "Unknown",
			serverownername: "Unknown",
			serverownernick: "Unknown",
			serverownerid: "Unknown",
			serverownerjoined: "Unknown",
			serverownerstatus: "Unknown",
			serverownergame: "Unknown",

			// Mention variables, 'mention' = First mentioned user

			mentionname: "Unknown",
			mentionid: "Unknown",
			mentiondiscrim: "Unknown",
			mentionstatus: "Unknown",
			mentiongame: "Unknown",
			mentionbot: "Unknown",
			mention: "Unknown",

			// Member variables of the mentioned user

			mentionjoined: "Unknown",
			mentionnick: "Unknown"
		}

		if(Message.member !== null){
			if(Message.member.nickname !== null){
				TagObject.snick = Message.member.nickname;
			}
		}

		if(Message.guild !== null){
			// Fill server variables
			let Guild = Message.guild;
			TagObject.serverregion = Guild.region||"Unknown";
			TagObject.servername = Guild.name||"Unknown";
			TagObject.serverid = Guild.id||"Unknown";
			TagObject.servermembs = Guild.memberCount||"Unknown";
			TagObject.serverchans = Guild.channels.size||"Unknown";
			TagObject.serverowner = Guild.owner||"Unknown";
			TagObject.serverdefchan = Guild.defaultChannel||"Unknown";
			TagObject.servercreated = Guild.createdAt.toUTCString()||"Unknown";
			TagObject.serververification = Guild.verificationLevel||"Unknown";
		}

		if(Message.mentions.users.size >= 1){
			let Mentions = Message.content.match(/<@!?\d+>/g);
			if(Mentions !== null){
				Mentions = Mentions.map(s => s.replace(/<@!?/, '').replace('>', ''));
				if(Mentions.length > 1){
					if(Mentions[0] === TagBot.user.id){
						Mentions.shift();
					}
				}else if(Mentions.length === 1){
					if(Mentions[0] === TagBot.user.id){
						Mentions.shift();
					}
				}

				if(Mentions.length >= 1){
					let Mention = Message.mentions.users.get(Mentions[0]);

					TagObject.mentionname = Mention.username;
					TagObject.mentionid = Mention.id;
					TagObject.mentiondiscrim = Mention.discriminator;
					TagObject.mentionstatus = Mention.presence.status;
					TagObject.mentiongame = Mention.presence.game||"Unknown";
					TagObject.mentionbot = Mention.bot;
					TagObject.mention = Mention;

					if(Message.guild !== null){
						// If it's not in a DM

						let Member = Message.guild.members.get(Mentions[0]);

						if(Member !== null && Member !== undefined){
							TagObject.mentionjoined = Member.joinedAt.toUTCString();
							TagObject.mentionnick = Member.nickname||Mention.username;
						}

					}
				}
			}
		}

		let template = generateTemplateString(Msg);

		let toSend = template(TagObject);
		toSend = toSend.chooseFormat();
		toSend = toSend.randFormat();
		toSend = new Date().format(toSend);

		Message.channel.sendMessage(toSend);
	}
}