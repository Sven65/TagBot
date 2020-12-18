
require('../Util/Extenders')

const config = require("config")

const User = require(`../Classes/User/User`)
const Tags = require(`../Classes/Tags/Tags`)

//const TH = require(`./Tags/Tags`)

const TagEngine = require(`../TagEngine`)

TagEngine.loadTags()

const getCommand = (Args, prefix=null, client, message) => {
	let command = ""
	let prefixType = -1
	prefix = prefix||client.Prefix

	if(Args[0].startsWith(prefix)){
		prefixType = 0
		command = Args[0].replace(prefix, "").toLowerCase()
	}else if(Args[0].toLowerCase() === client.user.username.toLowerCase()){
		prefixType = 1
		Args.shift()
	}else if(message.mentions.has(client.user.id)){
		prefixType = 2
		Args.splice(Math.max(Args.indexOf(`<@!${client.user.id}>`), Args.indexOf(`<@${client.user.id}>`)), 1)
	}

	if(prefixType >= 1){
		if(Args[0] === "" || Args[0] === undefined || Args[0] === null){
			return
		}

		command = Args[0].toLowerCase()
	}


	if(client.commands.Commands.Aliases[command] !== undefined){
		command = client.commands.Commands.Aliases[command]
	}

	return { command, prefixType }
}

/**
 * Checks if a variable is set
 * @param {*} variable The variable to check
 * @returns boolean
 */
const _isset = (variable) => {
	return variable !== null && variable !== undefined
}


async function _getPrefix(message){
	let prefix = config.get("prefix")

	if(message.channel.type === "text"){
		let prefixData = await message.client.cache.get(`DRPG:prefix:${message.guild.id}`)

		if(_isset(prefixData)){
			prefix = prefixData
		}
	}

	return await Promise.resolve(prefix)
}

const checkHasCooldown = async (client, message, command, now, cooldownTime = null) => {
	let last = now-1
	let firstTime = true

	let value = await client.cache.hget(`cooldowns:${message.author.id}`, command)
	if(value !== null){
		firstTime = false
		last = parseInt(value)
	}

	if (!cooldownTime) {
		cooldownTime = client.commands.resolveCooldown(command)
	}

	if(cooldownTime <= 0){
		cooldownTime = 1
	}

	if(message.author.id === config.get('owner')){
		last = now-cooldownTime*2000
	}

	if(now <= last+cooldownTime*1000 && !firstTime){
		let time = Math.ceil((last + cooldownTime * 1000 - now) / 1000)

		return { hasCooldown: true, timeLeft: time }
	}

	return { hasCooldown: false }
}

/**
 * Handles an incoming message and cooldowns
 * @function
 * @param {Message} message - The Discord.js message to handle
 * @author Mackan
 */
module.exports = async (message) => {

	if(message.author.bot || !message.hasPerm("SEND_MESSAGES")){
		return
	}

	const client = message.client
	const tagClass = new Tags()

	if(message.channel.type === "text"){
		if(config.get('ignoredChannels').indexOf(message.channel.id) > -1 && message.author.id !== config.get("owner")){
			return
		}
	}

	if(config.get("debug")){
		const testers = config.get("testers")

		if(testers.indexOf(message.author.id) <= -1){
			return
		}
	}

	let Args = message.content.replace(/\s\s+/g, " ").split(" ")

	let serverData = null

	if(serverData === null){
		serverData = { prefix: await _getPrefix(message) }
	}

	if(config.get("debug")){
		serverData.prefix = config.get("prefix")
	}

	let cmdData = getCommand(Args, serverData.prefix, client, message)
	let prefixType = cmdData.prefixType

	if(prefixType <= -1){
		return
	}

	const user = new User(message.author)

	// Check if user is ignored

	const isIgnored = await user.isIgnored()

	if(isIgnored){
		return
	}

	if (!cmdData) {
		return
	}

	let command = cmdData.command

	if(client.commands.Commands.Aliases[command] !== undefined){
		command = client.commands.Commands.Aliases[command]
	}

	let now = new Date().valueOf()
	let cooldown

	Args.shift()


	if(client.commands.Commands.All.indexOf(command) <= -1){
		// Check if tag exists
		console.log("tag 2")

		const tag = await tagClass.getTag(command)

		if (!_isset(tag)) {
			return
		}

		cooldown = await checkHasCooldown(client, message, `tag:${command}`, now, config.get('tagCooldown'))

		if (cooldown.hasCooldown) {
			message.channel.send(`You need to calm down, ${message.author.username}. :hourglass: ${cooldown.timeLeft} second${cooldown.timeLeft>1?'s':''}`)
			return
		} else {
			await client.cache.hset(`cooldowns:${message.author.id}`, `tag:${command}`, now)
			TagEngine.handle(Args, message, tag)
		}

		return
	}




	cooldown = await checkHasCooldown(client, message, command, now)

	if (cooldown.hasCooldown) {
		message.channel.send(`You need to calm down, ${message.author.username}. :hourglass: ${cooldown.timeLeft} second${cooldown.timeLeft>1?'s':''}`)
		return
	} else {
		await client.cache.hset(`cooldowns:${message.author.id}`, command, now)
		const cmd = client.commands.resolveCommand(command)
		cmd.Execute(Args, message).catch(error => message.sendError(error))
	}
}