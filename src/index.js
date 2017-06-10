const rethinkdb = require("rethinkdb")
const commandHandler = require(`${__dirname}/Commands/Handler.js`)
const messageHandler = require(`${__dirname}/Handlers/messageCreate.js`)


const TBClient = require(__dirname+"/TagBot.js")
const TagBot = new TBClient()
const commands = new commandHandler(`${__dirname}/Commands/Commands`)

rethinkdb.connect({host: TagBot.Config.db.url, port: TagBot.Config.db.port, user: TagBot.Config.db.user, password: TagBot.Config.db.pass}, (err, conn) => {
	console.log("Connected to rethink")
	if(err){ throw err; }

	conn.use(TagBot.Config.db.db)
	global.ReDB = {r: rethinkdb, conn: conn}

	commands.loadCommands().then(() => {
		TagBot.commands = commands

		TagBot.login().then(() => {
			// Logged in
			console.log("Logged in");
		}).catch(e => {
			throw e
		})

		TagBot.on('error', (error) => {
			console.dir(error);
		});

		TagBot.on("warn", (str) => {
			console.log(str);
		});

		TagBot.on('message', (message) => {
			try{
				messageHandler(message)
			}catch(e){
				console.log(e)
			}
		})
	}).catch(e => {throw e})
})

process.once('uncaughtException', (err) => {
	console.dir("got error "+err.stack);
	console.dir("UNCAUGHT")
	console.log("MESSAGE", err.message)
	console.log("FILENAME", err.fileName)
	console.log("LINE", err.lineNumber)
	console.log("\n\nSTACK", err)
	setTimeout(() => {
		process.exit(0);
	}, 2500)
})

process.on('unhandledRejection', (reason, p) => {
	console.log("Unhandled Rejection at ", p, 'reason ', reason.stack);
})