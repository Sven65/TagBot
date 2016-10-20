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

const redis = require("redis");
let client = redis.createClient();
let tags = {};

let Tags = {
	loadTags: () => {
		return new Promise((resolve, reject) => {
			client.get("tagbot:tags", (err, data) => {
				if(err){ reject(err); }
				tags = JSON.parse(data);
				resolve(true);
			});
		});
	},

	getTag: (tag) => {
		return tags[tag].content||null;
	},
	editTag: (tag, data) => {
		return new Promise((resolve, reject) => {
			tags[tag] = data;
			client.set("tagbot:tags", JSON.stringify(tags), (err) => {
				if(err){reject(err);}
				resolve(true);
			});
		});
	},
	deleteTag: (tag) => {
		return new Promise((resolve, reject) => {
			delete tags[tag];
			client.set("tagbot:tags", JSON.stringify(tags), (err) => {
				if(err){reject(err);}
				resolve(true);
			});
		});
	},
	addTag: (tag, data) => {
		return new Promise((resolve, reject) => {
			tags[tag] = data;
			client.set("tagbot:tags", JSON.stringify(tags), (err) => {
				if(err){reject(err);}
				resolve(true);
			});
		});
	},
	list: (user) => {
		return new Promise((resolve, reject) => {
			let q = [];
			Object.keys(tags).map((a) => {
				if(tags[a].owner === user){
					q.push(a);
				}
			});
			resolve(q);
		});
	},
	tagExist: (tag) => {
		return tags.hasOwnProperty(tag);
	}
}