Object.defineProperty(Number.prototype, "formatNumber", {
	enumerable: false,
	writable: true,
	value: function(){
		return this.toString().replace(/(\d)(?=(\d{3})+(?!\d))/g, "$1,")
	}
})

Object.defineProperty(String.prototype, "formatNumber", {
	enumerable: false,
	writable: true,
	value: function(){
		return this.toString().replace(/(\d)(?=(\d{3})+(?!\d))/g, "$1,")
	}
})

Object.defineProperty(String.prototype, "formatUnicorn", {
    enumerable: false,
    writable: true,
    value: function(){
        let str = this.toString()
        if(arguments.length){
            let t = typeof arguments[0]
            let key
            let args = ("string" === t || "number" === t) ?
                Array.prototype.slice.call(arguments)
                : arguments[0]
            for(key in args){
                str = str.replace(new RegExp("\\{" + key + "\\}", "gi"), args[key])
            }

        }
        return str
    }
})

Object.defineProperty(Array.prototype, "random", {
	enumerable: false,
	writable: true,
	value: function(){
		return this[Math.floor(Math.random() * ((this.length-1) - 0 + 1)) + 0]
	}
})

Object.defineProperty(Array.prototype, "rInt", {
	enumerable: false,
	writable: true,
	value: function(){
		return Math.floor(Math.random()*(this[1]-this[0] +1))+this[0]
	}
})

Object.defineProperty(String.prototype, "chooseFormat", {
	enumerable: false,
	writable: true,
	value: function(){
		let string = this.toString();
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
})

Object.defineProperty(String.prototype, "randFormat", {
	enumerable: false,
	writable: true,
	value: function(){
		let string = this.toString();
		let matches = string.match(/\{rint\(.*?[0-9]?\)\}/g);
		
		if(matches !== null){
			let numbers = matches.map(match => {
				const parsed = match.match(/(\d+),\s*(\d+)/)
	
				if (parsed) {
					const n1 = parseInt(parsed[1], 10)
					const n2 = parseInt(parsed[2], 10)
	
					return [n1, n2].rInt()
				}
			});
	
			numbers.forEach(number => {
				string = string.replace(/\{rint\(.*?[0-9]?\)\}/,number);
			});
		}
		return string;
	}
})

Object.defineProperty(Date.prototype, "format", {
	enumerable: false,
	writable: true,
	value: function(string){
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
		}

		for(let x in format){
			let reg = new RegExp(x, "g");
			string = string.replace(reg, format[x]);
		}
		return string;
	}
})

Object.defineProperty(String.prototype, "capFirst", {
	enumerable: false,
	writable: true,
	value: function(){
		return this.charAt(0).toUpperCase() + this.slice(1);
	}
})
