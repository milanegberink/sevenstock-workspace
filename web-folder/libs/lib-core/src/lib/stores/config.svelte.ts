import { browser } from '$app/environment';

class Config {
	sidebar: { width: number; open: boolean } = $state({
		open: true,
		openWidth: 250,
		get width() {
			return this.open ? this.openWidth : 56;
		}
	});
	constructor() {
		this.load();
	}
	load() {
		if (browser) {
			// const saved = localStorage.getItem('config');
			// if (saved) {
			// 	Object.assign(this, JSON.parse(saved));
			// 	return;
			// }
			this.sidebar.width = !this.sidebar.open ? 54 : 250;
		}
	}
}

export const config = new Config();
