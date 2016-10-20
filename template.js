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

String.prototype.format = function(){
	var args = arguments;
	this.unkeyed_index = 0;
	return this.replace(/\{(\d*)\}/gmi, function(match, key) { 
		if (key === '') {
			key = this.unkeyed_index;
			this.unkeyed_index++
		}
		if (key == +key) {
			return args[key] !== 'undefined'
				? args[key]
				: match;
		} else {
			for (var i = 0; i < args.length; i++) {
				if (typeof args[i] === 'object' && typeof args[i][key] !== 'undefined') {
					return args[i][key];
				}
			}
			return match;
		}
	}.bind(this));
};

Array.prototype.rInt = function(){
	var min = Number(this[0]);
	var max = Number(this[1]);
	return Math.floor(Math.random() * (max - min + 1)) + min;
}

Array.prototype.random = function(){
	return this[Math.floor(Math.random() * ((this.length-1) - 0 + 1)) + 0];
}

var generateTemplateString = (function(){
	var cache = {};

	function generateTemplate(template){

	var fn = cache[template];

	if (!fn){

	// Replace ${expressions} (etc) with ${map.expressions}.

	var sanitized = template
		.replace(/\$\{([\s]*[^;\s]+[\s]*)\}/g, function(_, match){
			return `\$\{map.${match.trim()}\}`;
		})
		// Afterwards, replace anything that's not ${map.expressions}' (etc) with a blank string.
		.replace(/(\$\{(?!map\.)[^}]+\})/g, '');

	fn = Function('map', `return \`${sanitized}\``);

	}

	return fn;
};

return generateTemplate;
})();

function formatDate(time){
	var newDate = new Date();
	newDate.setTime(time);
	return newDate.toUTCString();
}

function chooseFormat(sstring){
	let matches = sstring.match(/\{choose\((.*?)\)\}/g);
	if(matches !== null){
		let numbers = matches.map(match => {
			let m = match.match(/[^|]+/g).length;
			return match.match(/[^|]+/g)[Math.floor(Math.random()*m)];
		});

		console.log(matches,numbers);

		numbers.forEach(number => {
			sstring = sstring.replace(/(\{choose\(.*?\)\})/,number);
		});
	}
	return sstring;
}

function rInt(min, max){
	return Math.floor(Math.random() * (max - min + 1)) + min;
}

function rformat(sstring){
	let matches = sstring.match(/\{rint\(.*?[0-9]?\)\}/g);
	if(matches !== null){
		let numbers = matches.map(match => {
			let m = match.match(/\d/g).length;
			return match.match(/\d/g).rInt();
		});

		console.log(matches,numbers);

		numbers.forEach(number => {
			sstring = sstring.replace(/\{rint\(.*?[0-9]?\)\}/,number);
		});
	}
	return sstring;
};

module.exports = function(msg, message){
	var x = message.content.split(" ");
	var formt = x.splice(1, x.length);

	var xObj = {};

	for(i=0;i<formt.length;i++){
		xObj[Number(i)] = formt[i];
	}

	var q = msg.format(formt);

	q = chooseFormat(q);
	q = rformat(q);//.rformat();

	var tObj = {
		
		// Sender variables

		sname: message.author.username,
		sid: message.author.id,
		sdiscrim: message.author.discriminator,
		sstatus: message.author.status,
		sbot: message.author.bot,
		sender: message.author,
		// Details of sender on a server
		sjoined: "",
		// Message variables
		mtime: message.timestamp,
		mtimeh: formatDate(message.timestamp),
		mid: message.id,
		mcont: message.content,
		mcleancont: message.cleanContent,
		// Channel variables
		chanid: "",
		channame: "",
		chantype: "",
		chanpos: 0,
		chantopic: "",
		// Server variables
		serverregion: "",
		servername: "",
		serverid: "",
		servermembs: 0,
		serverchans: 0,
		serverowner: "",
		serverdefchan: "",
		// Mentions
		mentionname: "",
		mentionid: "",
		mentiondiscrim: "",
		mentionstatus: "",
		mentionbot: "",
		mention: "",
		mentionjoined: "",
		// Misc variables
		tags: ""
	};

	if(message.author.game == null){
		tObj["sgame"] = "";
		if(msg.trim().toLowerCase() == "${sgame}"){
			tObj["sgame"] = "Nothing";
		}
	}else{
		tObj["sgame"] = message.author.game.name;
	}

	if(message.channel.type === "text"){
		tObj["chanid"] = message.channel.id;
		tObj["channame"] = message.channel.name;
		tObj["chantype"] = message.channel.type;
		tObj["chanpos"] = message.channel.position;
		tObj["chantopic"] = message.channel.topic;

		if(message.mentions.users.length >= 1){
			tObj["mentionname"] = message.mentions.users[0].username;
			tObj["mentionid"] = message.mentions.users[0].id;
			tObj["mentiondiscrim"] = message.mentions.users[0].discriminator;
			tObj["mentionstatus"] = message.mentions.users[0].status;
			tObj["mentionbot"] = message.mentions.users[0].bot;
			tObj["mention"] = message.mentions.users[0];
			tObj["mentionjoined"] = server.members.find("id", message.mentions.users[0].id).joinDate;
		}

		if(message.guild != undefined){
			var server = message.guild;
			tObj["sjoined"] = server.members.find("id", message.author.id).joinDate;
			tObj["serverregion"] = server.region;
			tObj["serverid"] = server.id;
			tObj["servername"] = server.name;
			tObj["servermembs"] = server.memberCount;
			tObj["serverchans"] = server.channels.size;
			tObj["serverowner"] = server.owner.user.username;
			tObj["serverdefchan"] = server.defaultChannel.name;
		}
	}


	var togs = [];
	Object.keys(tObj).map((a) => {
		togs.push("``"+a+"``");
	});

	tObj["tags"] = togs.sort().join(", ");

	//tObj

	var tmpl = generateTemplateString(q);

	return tmpl(tObj);
}
