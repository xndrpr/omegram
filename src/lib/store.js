import { invoke } from "@tauri-apps/api";
import { writable } from "svelte/store";

export const endpoint = writable("/");
export const auth = writable(false);
export const dialogs = writable([]);
export const messages = writable([]);

async function initializeStores() {
    const dialogsData = await invoke("get_setting", { key: "dialogs" });
    dialogs.set(JSON.parse(dialogsData || "[]"));

    const messagesData = await invoke("get_setting", { key: "messages" });
    messages.set(JSON.parse(messagesData || "[]"));
}

initializeStores();
