/** @typedef HelloMsg
 * @prop {string} msg
 */

/** @type HelloMsg */
const message = {
	msg: "world"
};

console.log("hello", message.msg);

/**
 * @param {string} id
 * @return void
 */
function clearInnerHtml(id) {
	const element = document.getElementById(id);
	if (element && element.innerHTML) {
		element.innerHTML = "";
	}
}

/**
 * @param {string} id
 * @return void
 */
function clearValue(id) {
	const element = document.getElementById(id);
	if (element && element.value) {
		element.value = "";
	}
}

/**
 * @param {*} event
 * @return void
 */
function testAfterRequest(event) {
	console.log('event', event);
}
