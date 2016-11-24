String.prototype.format = function(){
	let args = arguments[0];
	this.unkeyed_index = 0;
	return this.replace(/\{(\d*)\}/gmi, function(match, key){ 
		if(key === ''){
			key = this.unkeyed_index;
			this.unkeyed_index++;
		}
		if(key == +key){
			return args[key] !== 'undefined'?args[key]:match;
		}else{
			for(let i=0;i<args.length;i++){
				if(typeof args[i] === 'object' && typeof args[i][key] !== 'undefined'){
					return args[i][key];
				}
			}
			return match;
		}
	}.bind(this));
};

Array.prototype.rInt = function(){
	return Math.floor(Math.random()*(this[1]-this[0] +1))+this[0];
}

Array.prototype.random = function(){
	return this[Math.floor(Math.random() * ((this.length-1) - 0 + 1)) + 0];
}

String.prototype.chooseFormat = function(){
	let string = this;
	let matches = string.match(/\{choose\((.*?)\)\}/g);
	if(matches !== null){
		let numbers = matches.map(match => {
			let m = match.match(/[^|]+/g).length;
			return match.match(/[^|]+/g)[Math.floor(Math.random()*m)].replace(/\)}$/, "").replace(/^{choose\(/, "");
		});

		numbers.forEach(number => {
			string = string.replace(/(\{choose\(.*?\)\})/, number);
		});
	}
	return string;
}

String.prototype.randFormat = function(){
	let string = this;
	let matches = string.match(/\{rint\(.*?[0-9]?\)\}/g);
	if(matches !== null){
		let numbers = matches.map(match => {
			let m = match.match(/\d/g).length;
			return match.match(/\d/g).rInt();
		});

		numbers.forEach(number => {
			string = string.replace(/\{rint\(.*?[0-9]?\)\}/,number);
		});
	}
	return string;
}

Date.prototype.format = function(string){
	let format = {
		"%%": "%",
		"%a": ["Sun", "Mon", "Tue", "Wed", "Fri", "Sat"][this.getDay()],
		"%A": ["Sunday", "Monday", "Tuesday", "Wednesday", "Friday", "Saturday"][this.getDay()],
		"%b": ["Jan", "Feb", "March", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"][this.getMonth()],
		"%B": ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"][this.getMonth()],
		"%d": this.getDate(),
		"%D": this.getMonth()+1+"/"+this.getDate()+"/"+this.getFullYear().toString().substring(2, 4),
		"%H": this.getHours(),
		"%m": this.getMonth()+1,
		"%M": this.getMinutes(),
		"%S": this.getSeconds(),
		"%T": this.getHours()+":"+this.getMinutes()+":"+this.getSeconds(),
		"%y": this.getFullYear().toString().substring(2, 4),
		"%Y": this.getFullYear()
	};
	for(let x in format){
		let reg = new RegExp(x, "g");
		string = string.replace(reg, format[x]);
	}
	return string;
}