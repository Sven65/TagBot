class Cache{

	/** Creates a new cache
     * @constructs Cache
     * @memberof Util
     * @param {RedisCli} redis - The redis client to use
     * @author Mackan
	 */
	constructor(redis){
		this.client = redis
	}

	/** Sets a value
	 * @function
	 * @param {String} key - The ID of the key
	 * @param {String} value - The value to use
	 * @returns {Promise.<Object>}
	 * @memberof Util.Cache
	 * @author Mackan
	 */
	set(key, value){
		return new Promise((resolve, reject) => {
			this.client.set(key, value, (err, res) => {
				if(err){
					reject(err)
				}
				resolve(res)
			})
		})
	}

	/** Sets a hashmap
	 * @function
	 * @param {String} key - The ID of the key
	 * @param {Array} keyArray - The hashmap to use
	 * @returns {Promise.<Object>}
	 * @memberof Util.Cache
	 * @author Mackan
	 */
	hmset(key, keyArray){
		return new Promise((resolve, reject) => {
			this.client.HMSET(key, keyArray, (err, res) => {
				if(err){
					reject(err)
				}
				resolve(res)
			})
		})
	}

	/** Gets all keys in a hashmap
	 * @function
	 * @param {String} key - The key to get
	 * @returns {Promise.<Object>}
	 * @memberof Utils.Cache
	 * @author Mackan
	 */
	hgetAll(key){
		return new Promise((resolve, reject) => {
			this.client.hgetall(key, (err, res) => {
				if(err){
					reject(err)
				}
				resolve(res)
			})
		})
	}

	/** Sets a hashmap keys value
	 * @function
	 * @param {String} key - The ID of the key
	 * @param {String} hashkey - The hashkey to change
	 * @param {String} value - The value to set
	 * @returns {Promise.<Object>}
	 * @memberof Util.Cache
	 * @author Mackan
	 */
	hset(key, hashkey, value){
		return new Promise((resolve, reject) => {
			this.client.HSET(key, hashkey, value, (err, res) => {
				if(err){
					reject(err)
				}

				resolve(res)
			})
		})
	}

	/** Gets the value of a key
	 * @function
	 * @param {String} key - The ID of the key
	 * @returns {Promise.<Object>}
	 * @memberof Util.Cache
	 * @author Mackan
	 */
	get(key){
		return new Promise((resolve, reject) => {
			this.client.get(key, function(err, reply){
				if(err){
					reject(err)
				}
				resolve(reply)
			})
		})
	}

	/** Gets the value of a hashkey
	 * @function
	 * @param {String} key - The ID of the key
	 * @param {String} hashkey - The hashkey to get the value of
	 * @returns {Promise.<Object>}
	 * @memberof Util.Cache
	 * @author Mackan
	 */
	hget(key, hashkey){
		return new Promise((resolve, reject) => {
			this.client.HGET(key, hashkey, (err, value) => {
				if(err){
					reject(err)
				}

				resolve(value)
			})
		})
	}

	/** Deletes a key from redis
	 * @function
	 * @param {String} key - The key to delete
	 * @returns {Promise}
	 * @memberof Utils.Cache
	 * @author Mackan
	 */
	del(key){
		return new Promise((resolve, reject) => {
			this.client.del(key, (err, value) => {
				if(err){
					reject(err)
				}

				resolve(value)
			})
		})
	}

	/**
	 * Sets a key up to expire after a certain time
	 * @function
	 * @param {String} key - The key to expire
	 * @param {Number} ttl - The time until the key expires in seconds
	 * @returns {Promise}
	 * @memberof Utils.Cache
	 * @author Mackan
	 */
	expire(key, ttl){
		return new Promise((resolve, reject) => {
			this.client.expire(key, ttl, (err, value) => {
				if(err){
					reject(err)
				}

				resolve(value)
			})
		})
	}

	/**
	 * Gets a keys TTL
	 * @function
	 * @param {String} key - The key to get the TTL of
	 * @returns {Promise}
	 * @memberof Utils.Cache
	 * @author Mackan
	 */
	ttl(key){
		return new Promise((resolve, reject) => {
			this.client.ttl(key, (err, value) => {
				if(err){
					reject(err)
				}

				resolve(value)
			})
		})
	}

	/**
	 * Incremements a key by 1
	 * @function
	 * @param {String} key - The key to increment
	 * @returns {Promise}
	 * @memberof Utils.Cache
	 * @author Mackan
	 */
	incr(key){
		return new Promise((resolve, reject) => {
			this.client.incr(key, (err, value) => {
				if(err){
					reject(err)
				}

				resolve(value)
			})
		})
	}

	/**
	 * Gets if a key exists
	 * @function
	 * @param {String} key - The key to check
	 * @returns {Promise}
	 * @memberof Utils.Cache
	 * @author Mackan
	 */
	exists(key){
		return new Promise((resolve, reject) => {
			this.client.exists(key, (err, value) => {
				if(err){
					reject(err)
				}

				resolve(value===1)
			})
		})
	}

	/**
	 * Adds a member to a set
	 * @function
	 * @param {String} key - The key to set
	 * @param {Array} members - The members to add
	 * @returns {Promise}
	 * @memberof Util.Cache
	 * @author Mackan
	 */
	sadd(key, members){
		return new Promise((resolve, reject) => {
			this.client.sadd(key, members, (err, value) => {
				if(err){
					reject(err)
				}

				resolve(value)
			})
		})
	}

	/**
	 * Checks if a string is a member of a set
	 * @function
	 * @param {String} key - The key to set
	 * @param {String} member - The member to check
	 * @returns {Promise.<Boolean>}
	 * @memberof Util.Cache
	 * @author Mackan
	 */
	sismember(key, member){
		return new Promise((resolve, reject) => {
			this.client.sadd(key, member, (err, value) => {
				if(err){
					reject(err)
				}

				resolve(value===1)
			})
		})
	}

	/**
	 * Gets the amount of members in a set
	 * @function
	 * @param {String} key - The key to set
	 * @returns {Promise.<Number>}
	 * @memberof Util.Cache
	 * @author Mackan
	 */
	scard(key){
		return new Promise((resolve, reject) => {
			this.client.scard(key, (err, value) => {
				if(err){
					reject(err)
				}

				resolve(value)
			})
		})
	}

	/**
	 * Insert all the specified values at the tail of the list stored at key.
	 * @function
	 * @param {String} key - The key to push
	 * @param {*} values - The values to set
	 * @returns {Promise.<Number>}
	 * @memberof Util.Cache
	 * @author Mackan
	 */
	rpush(key, ...values){
		return new Promise((resolve, reject) => {
			this.client.rpush(key, values, (err, value) => {
				if(err){
					reject(err)
				}

				resolve(value)
			})
		})
	}

	/**
	 * Returns the specified elements of the list stored at key.
	 * @function
	 * @param {String} key - The key to get
	 * @param {Number} start - The index to start at
	 * @param {Number} stop - The index to stop at
	 * @returns {Promise.<Array>}
	 * @author Mackan
	 */
	lrange(key, start, stop){
		return new Promise((resolve, reject) => {
			this.client.lrange(key, start, stop, (err, value) => {
				if(err){
					reject(err)
				}

				resolve(value)
			})
		})
	}
}

module.exports = Cache