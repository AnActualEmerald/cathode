import {writable} from "svelte/store";

export let frames = writable(new Array<string>(4));
