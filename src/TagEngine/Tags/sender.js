class Sender {
	static tagType = 'multiple'

	static tagList = {
		sname: (message) => message.author.username,
		sid: (message) => message.author.id,
		sdiscrim: (message) => message.author.discriminator,
		sstatus: (message) => message.author.presence.status,
		sgame: (message) => message.author.presence.game || 'Unknown',
		sbot: (message) => message.author.bot,
		sender: (message) => `<@${message.author.id}>`,
	}
}

module.exports = Sender
