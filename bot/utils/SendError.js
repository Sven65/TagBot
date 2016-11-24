module.exports = (message, error) => {
	let Msg = ":x: An error occured!\n```js\n"+error.stack+"```";
	return message.channel.sendMessage(Msg);
};