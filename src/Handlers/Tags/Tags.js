class TagHandler{

	constructor(){

	}

	getSenderVariables(message){
		return {
			sname: message.author.username,
			sid: message.author.id,
			sdiscrim: message.author.discriminator,
			sstatus: message.author.presence.status,
			sgame: message.author.presence.game||"Unknown",
			sbot: message.author.bot,
			sender: `<@${message.author.id}>`
		}
	}

	getSendersMemberVariables(message){
		if(message.member !== null){
			return {
				sjoined: message.member.joinedAt.toUTCString(),
				snick: message.member.nickname||message.author.username
				
			}
		}else{
			return {
				sjoined: "Unknown",
				snick: message.author.username
			}
		}
	}

	getMessageVariables(message){
		return {
			mtime: message.createdAt.toUTCString(),
			mid: message.id
		}
	}

	getChannelVariables(message){
		return {
			chanid: message.channel.id,
			chantype: message.channel.type,
			chancreated: message.channel.createdAt.toUTCString(),
			channame: message.channel.name||"Unknown",
			chantopic: message.channel.topic||"Unknown"
		}
	}

	getServerVariables(message){
		if(message.guild !== null){
			return {
				serverregion: message.guild.region,
				servername: message.guild.name,
				serverid: message.guild.id,
				servermembs: message.guild.memberCount,
				serverchans: message.guild.channels.array().length,
				serverdefchan: message.guild.defaultChannel,
				servercreated: message.guild.createdAt.toUTCString(),
				serververification: message.guild.verificationLevel
			}
		}else{
			return {
				serverregion: "Unknown",
				servername: "Unknown",
				serverid: "Unknown",
				servermembs: "Unknown",
				serverchans: "Unknown",
				serverdefchan: "Unknown",
				servercreated: "Unknown",
				serververification: "Unknown"
			}
		}
	}

	getServerOwnerVariables(message){
		if(message.guild !== null){
			return {
				serverowner: message.guild.owner,
				serverownername: message.guild.owner.username,
				serverownernick: message.guild.owner.displayName,
				serverownerid: message.guild.owner.id,
				serverownerjoined: message.guild.owner.joinedAt.toUTCString(),
				serverownerstatus: message.guild.owner.presence.status,
				serverownergame: message.guild.owner.presence.game||"Unknown"
			}
		}else{
			return {
				serverowner: "Unknown",
				serverownername: "Unknown",
				serverownernick: "Unknown",
				serverownerid: "Unknown",
				serverownerjoined: "Unknown",
				serverownerstatus: "Unknown",
				serverownergame: "Unknown"
			}
		}
	}

	getMentionVariables(message){
				
		let toReturn = {
			mentionname: "Unknown",
			mentionid: "Unknown",
			mentiondiscrim: "Unknown",
			mentionstatus: "Unknown",
			mentiongame: "Unknown",
			mentionbot: "Unknown",
			mention: "Unknown",

			mentionjoined: "Unknown",
			mentionnick: "Unknown"
		}

		let Mentions = message.content.match(/<@!?\d+>/g)
		if(Mentions !== null){
			Mentions = Mentions.map(s => s.replace(/<@!?/, '').replace('>', ''))
			if(Mentions.length > 1){
				if(Mentions[0] === message.client.user.id){
					Mentions.shift()
				}
			}else if(Mentions.length === 1){
				if(Mentions[0] === message.client.user.id){
					Mentions.shift()
				}
			}

			if(Mentions.length >= 1){
				let Mention = message.mentions.users.get(Mentions[0])



				toReturn.mentionname = Mention.username;
				toReturn.mentionid = Mention.id;
				toReturn.mentiondiscrim = Mention.discriminator;
				toReturn.mentionstatus = Mention.presence.status;
				toReturn.mentiongame = Mention.presence.game||"Unknown";
				toReturn.mentionbot = Mention.bot;
				toReturn.mention = Mention;

				if(message.guild !== null){
					// If it's not in a DM

					let Member = message.guild.members.get(Mentions[0])

					if(Member !== null && Member !== undefined){
						toReturn.mentionjoined = Member.joinedAt.toUTCString()
						toReturn.mentionnick = Member.nickname||Mention.username
					}

				}
			}
			return toReturn
		}else{
			return toReturn
		}
	}

	Handle(Args, message, tag){
		let posArgs = {}
		Args.map((arg, i) => posArgs[i] = arg)

		let toSend = tag.content.formatUnicorn(posArgs)

		let tagObject =	Object.assign({},
			this.getSenderVariables(message),
			this.getSendersMemberVariables(message),
			this.getMessageVariables(message),
			this.getChannelVariables(message),
			this.getServerVariables(message),
			this.getServerOwnerVariables(message),
			this.getMentionVariables(message))

		toSend = toSend.formatUnicorn(tagObject)
		toSend = new Date().format(toSend)
		toSend = toSend.chooseFormat()
		toSend = toSend.randFormat()

		message.channel.send(toSend).catch(e => {
			console.error(e)
		})
	}
}

module.exports = TagHandler