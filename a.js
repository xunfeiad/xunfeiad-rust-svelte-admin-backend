// @ts-nocheck
class WebsocketClient {
	constructor(url) {
		this.url = url;
		this.websocket = new WebSocket(this.url);
	}
	onMessage() {
		this.websocket.onmessage = (event) => {
			console.log('receive a message: ' + event.data);
		};
		/**
		 *  document.addEventListener("message", event=>{
		 *  console.log("receive a message: " + event.data)
		 * })
		 */
	}

	onError() {
		this.websocket.onerror = (event) => {
			console.log('WebSocket error: ', event);
		};
	}
	onClose() {
		this.websocket.onclose = () => {
			console.log('websocket is closed.');
		};
	}
	send(data) {
		this.websocket.onopen = ()=>{
			this.websocket.send(data);
		}	
	}
	close() {
		this.websocket.close();
	}
}
