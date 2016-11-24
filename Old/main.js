/*
 Copyright Mackan <mackan@discorddungeons.me>

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

const Discord = require("discord.js");
const fs = require("fs")
const Config = require("./Config.json");
const cooldowns = require("./Cooldowns.json");
let Tags = require("./Tags.js");
let template = require("./template.js");
let commands = require("./Commands.js");



let bot = new Discord.Client({autoReconnect: true, disableEveryone: true});

let lastExecTime = {};


let firstTime = {};
Tags.loadTags().then(() => {
	global.Tags = Tags;
	bot.login(Config.token).then(() => {
		console.log("Logged in");
	}, console.log);
}).catch((e) => {
	throw e;
});


String.prototype.printf = function(){ 
	let num = arguments.length; 
	let oStr = this;   
	for(i=0;i<num;i++){ 
		let pattern = "\{"+(i)+"\}";
		let re = new RegExp(pattern, "gmi"); 
		oStr = oStr.replace(re, arguments[i]); 
	}
	return oStr;
}

function handle(tag, bot, message){
	try{
		let content = Tags.getTag(tag);
		if(content !== null){
			message.channel.sendMessage(template(content.content, message)).catch((e) => {
				console.dir(e);
			});
		}
		return;
	}catch(e){
		message.channel.sendMessage("Error! ```js\n"+e.stack+"```");
	}
}

function checkCooldown(user, cmd){
	if(lastExecTime[cmd] === undefined){
		lastExecTime[cmd] = {};
	}

	if(firstTime[cmd] === undefined){
		firstTime[cmd] = {};
	}

	let now = new Date().valueOf();
	let cd = 0;

	if(commands.cmds.indexOf(cmd) > -1){
		cd = cooldowns[cmd];
	}else{
		cd = cooldowns["tags"];
	}

	if(now < lastExecTime[cmd][user]+cd*1000 && firstTime[cmd].hasOwnProperty(message.author.id)){
		return {state: true, time: Math.round(((lastExecTime[cmd][message.author.id] + 2 * 1000) - now) / 1000)};
	}else{
		return {state: false};
	}
}

function saveTags(bot, message){
	client.set("tagbot:tags", JSON.stringify(tags));
}

bot.on("message", (message) => {
	try{

		if(message.author.id !== bot.user.id){

			let args = message.content.split(" ");

			let cmd;

			if(args[0].substring(0, Config.prefix.length) !== Config.prefix){
				if(args[0] === "<@"+bot.user.id+">" || args[0].toLowerCase() === bot.user.username.toLowerCase()){
					args = args.splice(1, args.length);

					if(args[0] === undefined){
						message.channel.sendMessage(`What do you want, ${message.author.username}?`);
						return;
					}
					cmd = args[0].toLowerCase();
				}else{
					return;
				}
			}else{
				cmd = args[0].replace(Config.prefix, "").toLowerCase();
			}

			if(commands.cmds.indexOf(cmd) > -1){
				commands[cmd].exec(bot, message, args);
			}else if(cmd === "eval" && message.author.id === Config.owner){
				try{
					let msg = "";
					if(args[1] === "-c"){
						args = args.splice(1, args.length);
						let code = args.splice(1, args.length).join(" ");
						msg += "```js\n"+code+"```\n";
						msg += "```js\n"+eval(code)+"```";
					}else{
						let code = args.splice(1, args.length).join(" ");
						msg += "```js\n"+eval(code)+"```";
					}
					message.channel.sendMessage(msg);
				}catch(e){
					message.channel.sendMessage("```js\n"+e+"```");
				}
			}else{

				if(Tags.tagExist(cmd)){
					let data = checkCooldown(message.author.id, cmd);
					if(data.state){
						message.channel.sendMessage(`${message.author.username}!, Please wait! ${data.time} seconds`, function(e, m){ bot.deleteMessage(m, {"wait": 6000}); });
						if (message.channel.type === "text") message.delete({"wait": 5000});
						return;
					}else{
						let now = new Date().valueOf();
						handle(cmd, bot, message);
						lastExecTime[cmd][message.author.id] = now;
						firstTime[cmd][message.author.id] = true;
					}
				}
			}
			return;
		}
	}catch(e){
		message.channel.sendMessage("```js\n"+e.stack+"```");
		return;
	}
});

bot.on("error",  err => console.log(err));

bot.on("warn", function(warn){
	console.log("[Warn] "+warn);
});
