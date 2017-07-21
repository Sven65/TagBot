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
			savatar: message.author.displayAvatarURL({format:'png'}),
			sbot: message.author.bot,
			sender: message.author.toString()
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
			mid: message.id,
			mtts: message.tts
		}
	}

	getChannelVariables(message){
		return {
			chanid: message.channel.id,
			chantype: message.channel.type,
			chancreated: message.channel.createdAt.toUTCString(),
			channame: message.channel.name||"Unknown",
			chantopic: message.channel.topic||"Unknown",
			chanpos: message.channel.position||"Unknown",
			channsfw: message.channel.nsfw||"Unknown"
		}
	}

	getServerVariables(message){
		if(message.guild !== null){
			return {
				serverregion: message.guild.region,
				servername: message.guild.name,
				servernameacr: message.guild.nameAcronym,
				serverafktimeout: message.guild.afkTimeout||"Unknown",
				servericon: message.guild.iconURL({format:'png'})||"None",
				serversplash: message.guild.aplashURL({format:'png'})||"None",
				serverid: message.guild.id,
				servermembs: message.guild.memberCount,
				serverchans: message.guild.channels.array().length,
				servervchans: message.guild.channels.array().filter(c => c.type === 'voice').length,
				servertchans: message.guild.channels.array().filter(c => c.type === 'text').length,
				servercreated: message.guild.createdAt.toUTCString(),
				serververification: message.guild.verificationLevel,
				servercontentfilter: message.guild.explicitContentFilter,

				serverdefchan: message.guild.defaultChannel,
				serverdefchanname: message.guild.defaultChannel.name||"Unknown",
				serverdefchantopic: message.guild.defaultChannel.topic||"Unknown",
				serverdefchanpos: message.guild.defaultChannel.position,
				serverdefchannsfw: message.guild.defaultChannel.nsfw
			}
		}else{
			return {
				serverregion: "Unknown",
				servername: "Unknown",
				servericon: "Unknown",
				serverid: "Unknown",
				servermembs: "Unknown",
				serverchans: "Unknown",
				servervchans: "Unknown",
				servertchans: "Unknown",
				serverdefchan: "Unknown",
				servercreated: "Unknown",
				serververification: "Unknown",
				serverdefchanname: "Unknown",
				serverdefchantopic: "Unknown",
				serverdefchanpos: "Unknown",
				serverdefchannsfw: "Unknown"
			}
		}
	}

	getServerOwnerVariables(message){
		if(message.guild !== null){
			return {
				serverowner: message.guild.owner,
				serverownername: message.guild.owner.user.username,
				serverownernick: message.guild.owner.displayName,
				serverownerid: message.guild.owner.user.id,
				serverownerjoined: message.guild.owner.joinedAt.toUTCString(),
				serverowneravatar: message.guild.owner.user.displayAvatarURL({format:'png'}),
				serverownerstatus: message.guild.owner.user.presence.status,
				serverownergame: message.guild.owner.user.presence.game||"Unknown",
				serverownerfrole: message.guild.owner.highestRole.name,
				serverownerhrole: message.guild.owner.hoistRole.name,
				serverownercrole: message.guild.owner.colorRole.name||"Unknown",
				serverownercolor: message.guild.owner.colorRole?message.guild.owner.displayHexColor:"None"
			}
		}else{
			return {
				serverowner: "Unknown",
				serverownername: "Unknown",
				serverownernick: "Unknown",
				serverownerid: "Unknown",
				serverownerjoined: "Unknown",
				serverownerstatus: "Unknown",
				serverownergame: "Unknown",
				serverownerfrole: "Unknown",
				serverownerhrole: "Unknown",
				serverownercrole: "Unknown",
				serverownercolor: "Unknown"
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
			mentionavatar: "Unknown",

			mentionjoined: "Unknown",
			mentionnick: "Unknown",
			mentionfrole: "Unknown",
			mentionhrole: "Unknown",
			mentioncrole: "Unknown",
			mentioncolor: "Unknown"
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
				toReturn.mentionavatar: Mention.author.displayAvatarURL({format:'png'});
				toReturn.mention = Mention;

				if(message.guild !== null){
					// If it's not in a DM

					let Member = message.guild.members.get(Mentions[0])

					if(Member !== null && Member !== undefined){
						toReturn.mentionjoined = Member.joinedAt.toUTCString();
						toReturn.mentionnick = Member.nickname||Mention.username;
						toReturn.mentionfrole = Member.highestRole.name;
						toReturn.mentionhrole = Member.hoistRole.name;
						toReturn.mentioncrole = Member.colorRole.name||"Unknown";
						toReturn.mentioncolor = Member.colorRole?message.guild.owner.displayHexColor:"None";
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
