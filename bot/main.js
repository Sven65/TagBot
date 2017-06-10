const Discord = require("discord.js");
const fs = require("fs");
const Config = require(`${__dirname}/Config.json`);
const rethink = require("rethinkdb");
const Handlers = require(`${__dirname}/handlers/index.js`);
const Extends = require(`${__dirname}/extends/extends.js`);
const SendError = require(`${__dirname}/utils/SendError.js`);
const tags = require(`${__dirname}/utils/tags/index.js`);
const TagHandler = require(`${__dirname}/tags/handler.js`);

let locked = false;

global.User = require(`${__dirname}/utils/user/index.js`);
global.Tags = new tags.Tags;


let Commands = {
	All: [],
	List: {},
	Map: {}
};


const CommandHelper = {
	resolveCommand: (command) => {
		return Commands.List[Commands.Map[command]][command];
	},
	resolveCooldown: (command) => {
		return Commands.List[Commands.Map[command]][command].cooldown;
	},
	loadCommands: () => {
		return new Promise((resolve, reject) => {
			try{
				let Files = fs.readdirSync(__dirname+"/commands");
				for(let File of Files){
					let stats = fs.lstatSync(__dirname+"/commands/"+File);
					if(!stats.isDirectory()){
						if(File.endsWith('.js')){
							try{
								if(Commands.List["Other"] === undefined){
									Commands.List["Other"] = {};
								}
								Commands.List["Other"][File.slice(0, -3).toLowerCase()] = require(__dirname+'/commands/'+File);
								Commands.All.push(File.slice(0, -3).toLowerCase());
								Commands.Map[File.slice(0, -3).toLowerCase()] = "Other";
								console.log("Loading "+File);
							}catch(e){
								console.dir(e);
								reject(e);
							}
						}
					}else{
						let DirFiles = fs.readdirSync(__dirname+"/commands/"+File);
						for(let DirFile of DirFiles){
							if(DirFile.endsWith('.js')){
								try{
									if(Commands.List[File] === undefined){
										Commands.List[File] = {};
									}
									Commands.List[File][DirFile.slice(0, -3).toLowerCase()] = require(__dirname+'/commands/'+File+"/"+DirFile);
									Commands.All.push(DirFile.slice(0, -3).toLowerCase());
									Commands.Map[DirFile.slice(0, -3).toLowerCase()] = File;
								}catch(e){
									console.dir(e);
									reject(e);
								}
							}
						}
					}
				}
				resolve();
			}catch(e){
				reject(e);
			}
		});
	}
}

let TagBot = new Discord.Client({
	autoReconnect: true,
	disableEveryone: true
});

global.TagBot = TagBot;

rethink.connect({
	host: "localhost",
	port: 28015,
	user: Config.db.user,
	password: Config.db.pass
}, (err, conn) => {
	if(err) throw err;
	conn.use("TagBot");
	TagBot.rdb = {r: rethink, conn: conn};
	TagBot.Config = Config;
	TagBot.SendError = SendError;
	TagBot.TagHandler = TagHandler;

	conn.on("close", () => {
		locked = true;
	})

	conn.on("timeout", () => {
		locked = true;
	})

	CommandHelper.loadCommands().then(() => {
		TagBot.Commands = Commands;
		TagBot.CommandHelper = CommandHelper;
		TagBot.login(Config.Token).then(() => {
			console.log("Logged in");
		}).catch((e) => {
			throw e;
		});
	}).catch((e) => {
		console.log(e.stack);
	});
});

TagBot.on("message", message => {
	if(locked) return;
	Handlers.messageCreate.Handle(message)
});