import { writable } from "svelte/store";

export const endpoint = writable("/")
export const auth = writable(false)
export const dialogs = writable(JSON.parse(localStorage.getItem("dialogs")) || []);
export const messages = writable(JSON.parse(localStorage.getItem("messages")) || []);