let generateTemplateString = (() => {
	let cache = {};
	function generateTemplate(template){
		let fn = cache[template];
		if(!fn){
			// Replace ${expressions} (etc) with ${map.expressions}.
			let sanitized = template.replace(/\$\{([\s]*[^;\s]+[\s]*)\}/g, (_, match) => {
				return `\$\{map.${match.trim()}\}`;
			}).replace(/(\$\{(?!map\.)[^}]+\})/g, '');
			fn = Function('map', `return \`${sanitized}\``);
		}
		return fn;
	};
	return generateTemplate;
})();

module.exports = {
	Handle: (Args, Message, Tag) => {
		let Msg = Tag.content.format(Args);
		Message.channel.sendMessage(Msg);
	}
}