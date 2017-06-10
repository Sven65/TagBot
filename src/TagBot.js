const Discord = require('discord.js')
const Config = require(`${__dirname}/Config.json`)

class TagBot extends Discord.Client{
	constructor(){
		super({
			autoReconnect: true,
			disableEveryone: true,
			messageCacheMaxSize: 5,
			messageCacheLifetime: 10,
			messageSweepInterval: 60,
			bot: true,
			disabledEvents: ["TYPING_START"]
		})

		this.Config = Config
	}

	login(){
		return super.login(this.Config.Token)
	}

	set rethinkdb(rethinkdb){
		this.rethinkdb = rethinkdb
	}

	get rethinkdb(){
		return this.rethinkdb
	}

	set commands(commands){
		this._commands = commands
	}

	get commands(){
		return this._commands
	}
	
	get Version(){
		return this.Config.version
	}

	//// FUNCTIONS \\\\\
	sendError(message, error){
		return message.channel.send(":x: An error occured\n```js\n"+error.stack+"``` :x:");
	}

	hasEmbedPerms(message){

		let embedPerms = false

		if(message.channel.type !== "text"){
			embedPerms = true;
		}else{
			if(message.channel.permissionsFor(this.user).has("EMBED_LINKS")){
				embedPerms = true;
			}
		}

		return embedPerms
	}
}

module.exports = TagBot