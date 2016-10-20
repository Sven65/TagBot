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
const fs = require("fs");
const redis = require("redis");
const config = require("./Config.json");
let template = require("./template.js");
let client = redis.createClient();

var bot = new Discord.Client({autoReconnect: true, disableEveryone: true});

var tags;
client.get("tagbot:tags", (err, data) => {
	tags = JSON.parse(data);
});
//var tags = require("./tags.json");
var lastExecTime = {};
var prefix = "£";
var blacklist = ["add", "edit", "reload", "delete", "list", "glist", "commands", "raw", "eval"];

setInterval(function(){
	lastExecTime = {};
}, 36000000);

var firstTime = {};

bot.login(config.token).then(() => {
	console.log("Logged in");
}, console.log);

String.prototype.printf = function(){ 
	var num = arguments.length; 
	var oStr = this;   
	for(i=0;i<num;i++){ 
		var pattern = "\{"+(i)+"\}";
		var re = new RegExp(pattern, "gmi"); 
		oStr = oStr.replace(re, arguments[i]); 
	}
	return oStr;
}

//printf('The lazy {0} {1} over the {2}', "dog", "jumps", "fox"); 

function handle(tag, bot, message){
	try{
		let msg = tags[tag].content;
		message.channel.sendMessage(template(msg, message)).catch((e) => {
			console.dir(e);
		});
		return;
	}catch(e){
		message.channel.sendMessage("Error! ```js\n"+e.stack+"```");
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

			if(args[0].substring(0, prefix.length) !== prefix){
				if(args[0] === "<@"+bot.user.id+">" || args[0].toLowerCase() === bot.user.username.toLowerCase()){
					args = args.splice(1, args.length);

					if(args[0] === undefined){
						message.channel.sendMessage("What do you want, "+message.author.username+" ?");
						return;
					}
					cmd = args[0].toLowerCase();
				}
			}else{
				cmd = args[0].replace(prefix, "").toLowerCase();
			}

			if(cmd === "add"){
				let tag = args[1];

				if(tag === undefined){
					message.channel.sendMessage("You what, "+message.author.username+"?");
				}

				let val = args.splice(2, args.length);
				if(tags.hasOwnProperty(tag.toLowerCase()) || blacklist.indexOf(tag.toLowerCase()) > -1){
					message.channel.sendMessage("Sorry, "+message.author.username+", but that tag already exists.");
					return;
				}else{
					let x = val.join(" ").replace(/\`/gmi, "\\`");
					if(x.length >= 3){
						tags[tag.toLowerCase()] = {"content": x,"owner":message.author.id};
						message.channel.sendMessage('Added tag '+tag+"!");
						saveTags();
						return;
					}else{
						message.channel.sendMessage(`Your tag needs to have atleast 5 characters of content, ${message.author.username}`);
					}
				}
			}else if(cmd === "edit"){
				if(args.length >= 3){
					let tag = args[1].toLowerCase();
					let val = args.splice(2, args.length);
					if(tags.hasOwnProperty(tag)){
						if(tags[tag].owner === message.author.id){
							tags[tag].content = val.join(" ").replace(/`/gmi, "\\`");
							message.channel.sendMessage("Changed tag ``"+tag+"``");
							saveTags(bot, message);
							return;
						}else{
							message.channel.sendMessage(message.author.username+", you don't own this tag.");
							return;
						}
					}else{
						message.channel.sendMessage("Unknown tag ``"+tag+"``");
						return;
					}
				}
			}else if(cmd == "reload" && message.author.id == owner){
				template = {};
				delete require.cache[require.resolve('./template.js')];
				template = require("./template.js");
				message.channel.sendMessage("Reloaded.");
				return;
			}else if(cmd == "delete"){
				if(args.length >= 2){
					var tag = args[1].toLowerCase();
					if(tags.hasOwnProperty(tag)){
						if(tags[tag]["owner"] == message.author.id){
							delete tags[tag];
							message.channel.sendMessage("Deleted tag ``"+tag+"``");
							saveTags(bot, message)
							return;
						}else{
							message.channel.sendMessage(message.author.username+", you don't own this tag.");
							return;
						}
					}else{
						message.channel.sendMessage("Unknown tag ``"+tag+"``");
						return;
					}
				}
			}else if(cmd == "list"){
				var list = [];
				var user = message.author.id;
				Object.keys(tags).map((a) => {
					if(tags[a]["owner"] == user){
						list.push("``"+a+"``");
					}
				});
				if(list.length <= 0){
					message.channel.sendMessage(message.author.username+", you don't own any tags.");
					return;
				}else{
					message.channel.sendMessage("Found "+list.length+" tags.\n"+list.sort().join(", "));
					return;
				}
			}else if(cmd == "glist"){
				var list = [];
				Object.keys(tags).map((a) => {
					list.push("``"+a+"``");
				});
				if(list.length <= 0){
					message.channel.sendMessage("No tags found :(");
					return;
				}else{
					message.channel.sendMessage("Found "+list.length+" tags.\n"+list.sort().join(", "));
					return;
				}
			}else if(cmd == "commands"){
				var list = [];
				var user = message.author.id;
				blacklist.map((a) => {
					list.push("``"+a+"``");
				});
				if(list.length <= 0){
					message.channel.sendMessage("No commands found :(");
					return;
				}else{
					message.channel.sendMessage("Found "+list.length+" commands.\n"+list.sort().join(", "));
					return;
				}
			}else if(cmd == "raw"){
				if(args.length >= 2){
					var tag = args[1].toLowerCase();
					if(tags.hasOwnProperty(tag)){
						var msg = tags[tag]["content"];
						message.channel.sendMessage("``"+msg+"``");
						return;
					}else{
						message.channel.sendMessage("Unknown tag ``"+tag+"``");
						return;
					}
				}
			}else if(cmd == "eval" && message.author.id == "141610251299454976"){
				try{
					var msg = "";
						if(args[1] == "-c"){
							args = args.splice(1, args.length);
							var code = args.splice(1, args.length).join(" ");
							msg += "```js\n"+code+"```\n";
							msg += "```js\n"+eval(code)+"```";
						}else{
							var code = args.splice(1, args.length).join(" ");
							msg += "```js\n"+eval(code)+"```";
						}
						message.channel.sendMessage(msg);
					}catch(e){
						message.channel.sendMessage("```js\n"+e+"```");
					}
			}else{

				if(tags.hasOwnProperty(cmd)){

					if(lastExecTime[cmd] == undefined){
						lastExecTime[cmd] = {};
					}

					if(firstTime[cmd] == undefined){
						firstTime[cmd] = {};
					}

					var now = new Date().valueOf();
					if(now < lastExecTime[cmd][message.author.id]+2*1000 && firstTime[cmd].hasOwnProperty(message.author.id)){
					
						message.channel.sendMessage(message.author.username.replace(/@/g, '@\u200b')+"!, Please wait! "+Math.round(((lastExecTime[cmd][message.author.id] + 2 * 1000) - now) / 1000) + " seconds", function(e, m){ bot.deleteMessage(m, {"wait": 6000}); });
						if (!message.channel.isPrivate) message.delete({"wait": 5000});
						return;
					}else{
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
