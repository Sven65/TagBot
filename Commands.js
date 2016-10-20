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

let Commands = {
	cmds: ["add", "edit", "delete", "list", "glist", "commands", "raw"],
	add: {
		exec: (bot, message, args) => {
			let tag = args[1];

			if(tag === undefined){
				message.channel.sendMessage(`You what, ${message.author.username}?`);
				return;
			}

			let val = args.splice(2, args.length);
			if(Tags.tagExist(tag.toLowerCase()) || Commands.cmds.indexOf(tag.toLowerCase()) > -1){
				message.channel.sendMessage(`Sorry, ${message.author.username}, but that tag already exists.`);
				return;
			}else{
				let x = val.join(" ").replace(/\`/gmi, "\\`");
				if(x.length >= 3){
					Tags.addTag(tag, {"content": x,"owner":message.author.id}).then(() => {
						message.channel.sendMessage(`Added tag ${tag}!`);
					}).catch((e) => {
						console.log(e);
					});
					return;
				}else{
					message.channel.sendMessage(`Your tag needs to have atleast 5 characters of content, ${message.author.username}`);
				}
			}
		}
	},
	edit: {
		exec: (bot, message, args) => {
			if(args.length >= 3){
				let tag = args[1].toLowerCase();
				let val = args.splice(2, args.length);
				if(Tags.tagExist(tag)){
					let tg = Tags.getTag(tag);
					if(tg.owner === message.author.id){
						Tags.editTag(tag, {content: val.join(" ").replace(/`/gmi, "\\`"), owner: message.author.id}).then(() => {
							message.channel.sendMessage("Changed tag ``"+tag+"``");
						}).catch((e) => {
							console.log(e);
						});
						return;
					}else{
						message.channel.sendMessage(`You don't own this tag, ${message.author.username}.`);
						return;
					}
				}else{
					message.channel.sendMessage("Unknown tag ``"+tag+"``");
					return;
				}
			}
		}
	},
	delete: {
		exec: (bot, message, args) => {
			if(args.length >= 2){
				let tag = args[1].toLowerCase();
				if(Tags.exists(tag)){
					let tg = Tags.getTag(tag);
					if(tg.owner === message.author.id){
						Tags.deleteTag(tag).then(() => {
							message.channel.sendMessage("Deleted tag ``"+tag+"``");
						}).catch((e) => {
							console.log(e);
						});
						return;
					}else{
						message.channel.sendMessage(`You don't own this tag, ${message.author.username}.`);
						return;
					}
				}else{
					message.channel.sendMessage("Unknown tag ``"+tag+"``");
					return;
				}
			}
		}
	},
	list: {
		exec: (bot, message, args) => {
			Tags.list(message.author.id).then((t) => {
				let b = [];
				t.map((a) => {
					b.push(``+a+``);
				});
				let msg = b.join(', ');
				if(msg.length > 2000){
					msg = `Too many tags to display. (${b.length})`;
				}
				message.channel.sendMessage(msg);
			});
		}
	},
	glist: {
		exec: (bot, message, args) => {
			Tags.glist().then((t) => {
				let b = [];
				t.map((a) => {
					b.push(``+a+``);
				});
				let msg = b.join(', ');
				if(msg.length > 2000){
					msg = `Too many tags to display. (${b.length})`;
				}
				message.channel.sendMessage(msg);
			});
		}
	},
	commands: {
		exec: (bot, message, args) => {
			let list = [];
			this.cmds.map((a) => {
				list.push("``"+a+"``");
			});
			if(list.length <= 0){
				message.channel.sendMessage("No commands found :(");
				return;
			}else{
				message.channel.sendMessage(`Found ${list.length} commands.\n${list.sort().join(", ")}`);
				return;
			}
		}
	},
	raw: {
		exec: (bot, message, args) => {
			if(args.length >= 2){
				let tag = args[1].toLowerCase();
				if(Tags.tagExist(tag)){
					let t = Tags.getTag(tag);
					let msg = t.content;
					message.channel.sendMessage("``"+msg+"``");
					return;
				}else{
					message.channel.sendMessage("Unknown tag ``"+tag+"``");
					return;
				}
			}
		}
	}
};

module.exports = Commands;