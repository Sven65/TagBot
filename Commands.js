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
		exec: (bot, message) => {
			let tag = args[1];

			if(tag === undefined){
				message.channel.sendMessage(`You what, ${message.author.username}?`);
				return;
			}

			let val = args.splice(2, args.length);
			if(Tags.tagExist(tag.toLowerCase()) || this.cmds.indexOf(tag.toLowerCase()) > -1){
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
		exec: (bot, message) => {
			
		}
	}

};

module.exports = Commands;