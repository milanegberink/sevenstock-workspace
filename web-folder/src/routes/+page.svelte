<script>
	const req = new Request('https://jsonplaceholder.typicode.com/todos/1');

	async function test() {
		let res = await fetch(req);
		let x = await res.json();
	}
</script>

<button onclick={test}>Click me</button>

<a href="/test">test</a>
