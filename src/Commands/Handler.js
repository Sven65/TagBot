const fs = require("fs")

class CommandHandler{
	constructor(directory){
		this.Commands = {
			All: [],
			List: {},
			Map: {},
			Aliases: {}
		}

		this.commandDirectory = directory
	}

	loadCommands(){
		return new Promise((resolve, reject) => {
			try{
				const Files = fs.readdirSync(this.commandDirectory)
				for(const File of Files){
					let stats = fs.lstatSync(`${this.commandDirectory}/${File}`);

					if(!stats.isDirectory()){
						if(File.endsWith('.js')){
							try{
								if(this.Commands.List.Other === undefined){
									this.Commands.List.Other = {};
								}

								let cmdReq = require(`${this.commandDirectory}/${File}`)

								console.log(`Loading ${File}`)

								let cmd = new cmdReq()

								this.Commands.List.Other[File.slice(0, -3).toLowerCase()] = cmd
								this.Commands.All.push(File.slice(0, -3).toLowerCase());
								this.Commands.Map[File.slice(0, -3).toLowerCase()] = "Other";

								if(cmd.Metadata.hasOwnProperty("Alias")){
									cmd.Metadata.Alias.map(alias => {
										this.Commands.Aliases[alias] = File.slice(0, -3).toLowerCase();
									});
								}

							}catch(e){
								console.log(`error file: ${File}`)
								console.dir(e);
								reject(e);
							}
						}
					}else{
						const DirFiles = fs.readdirSync(`${this.commandDirectory}/${File}`);
						for(const DirFile of DirFiles){
							if(DirFile.endsWith('.js')){
								try{
									if(this.Commands.List[File] === undefined){
										this.Commands.List[File] = {};
									}

									console.log(`Loading ${DirFile}`)
									let cmdReq = require(`${this.commandDirectory}/${File}/${DirFile}`)

									let cmd = new cmdReq()

									this.Commands.List[File][DirFile.slice(0, -3).toLowerCase()] = cmd;
									this.Commands.All.push(DirFile.slice(0, -3).toLowerCase());
									this.Commands.Map[DirFile.slice(0, -3).toLowerCase()] = File;

									if(cmd.Metadata.hasOwnProperty("Alias")){
										cmd.Metadata.Alias.map(alias => {
											this.Commands.Aliases[alias] = DirFile.slice(0, -3).toLowerCase();
										});
									}
								}catch(e){
									console.log(`error dirfile: ${DirFile}`)
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

	/** Resolves a command and returns it
	 * @function
	 * @param {string} command - The command to resolve
	 * @returns {Command}
	 */
	resolveCommand(command){
		return this.Commands.List[this.Commands.Map[command]][command];
	}

	/** Resolves a command and returns it's cooldown
	 * @function
	 * @param {string} command - The command to resolve
	 * @returns {number} The cooldown of the command
	 */
	resolveCooldown(command){
		return this.Commands.List[this.Commands.Map[command]][command].Metadata.cooldown;
	}
}

module.exports = CommandHandler