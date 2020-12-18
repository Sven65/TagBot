const config = require("config")
const redis = require("redis")
const TagBotClient = require("./Client.js")

const commandHandler = require(`./Commands/Handler.js`)
const messageHandler = require(`./Handlers/messageCreate.js`)

const Cache = require(`./Util/Cache`)

const Client = new TagBotClient()
const commands = new commandHandler(`${__dirname}/Commands/Commands`)


const redisCli = redis.createClient({
	host: config.get('redis.host'),
	port: config.get('redis.port'),
})

redisCli.on("error", err => {
	throw err
})

const cache = new Cache(redisCli)

const r = require("rethinkdbdash")({
	pool: false,
})

async function init () {
	let connection
	try {
		connection = await r.connect(config.get('rethink'))
		console.log("\u001b[92mConnected to rethink\u001b[39m")
	} catch (e) {
		console.error('Failed to connect to rethink: ', e)
		throw e
	}

	const ReDB = {
		r: r,
		conn: connection,
	}

	global.ReDB = ReDB

	await commands.loadCommands()

	Client.commands = commands
	Client.cache = cache


	Client.login()

	Client.on("message", (message) => {
		messageHandler(message)
	})
}

init()
