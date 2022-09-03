<script lang="ts">
	import { onMount } from "svelte";
	import { appWindow } from "@tauri-apps/api/window";

	//Replace with a store in the future
	const images = [
		"images/1.png",
		"images/3.png",
		"images/2.png",
		"images/4.png",
	];

	let src = "";
	let buf = 0;
	let open = false;
	let closed = false;
	let blink = false;
	let inAnim = "jump-in";
	let outAnim = "none";

	$: {
		if (buf > 50) {
			open = true;
			closed = false;
		} else {
			open = false;
			closed = true;
		}
	}

	$: {
		if (closed) {
			src = images[0];
		} else if (open) {
			src = images[2];
		}

		if (blink && closed) {
			src = images[1] ? images[1] : images[0];
		} else if (blink && open) {
			src = images[3] ? images[3] : images[2];
		}
	}

	onMount(async () => {
		await appWindow.listen("mouth-open", () => {
			buf = 100;
		});

		await appWindow.listen("mouth-close", () => {
			buf = buf - 10;
			buf = buf < 0 ? 0 : buf;
		});

		await appWindow.listen("blink", () => {
			blink = true;
			setTimeout(() => (blink = false), 300);
		});
	});
</script>

<img {src} alt="tuber" class:open class:closed class="{inAnim} {outAnim}" />

<style lang="scss">
	@keyframes jump-out {
		0% {
			transform: translateY(-50%);
		}

		50% {
			transform: translateY(-52%);
		}

		100% {
			transform: translateY(-50%);
		}
	}

	@keyframes jump-in {
		0% {
			transform: translateY(-50%);
		}

		50% {
			transform: translateY(-52%);
		}

		100% {
			transform: translateY(-50%);
		}
	}

	img {
		position: absolute;
		transform: translateY(-50%);
		top: 50vh;
		left: calc(50vw - 200px);
		width: 400px;
	}

	.closed.jump-out {
		animation: jump-out;
		animation-duration: 0.2s;
	}

	.open.jump-in {
		animation: jump-in;
		animation-duration: 0.2s;
	}
</style>
