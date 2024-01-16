<script>
  import { onMount } from "svelte";
  import Login from "./pages/Login.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { endpoint } from "./lib/store";

  onMount(async () => {
    auth = await invoke("check_auth");
    auth = auth;
    if (auth == false) {
      switchEndpoint("/login");
    } else {
      dialogs = JSON.parse(localStorage.getItem("dialogs"));
      dialogs = await invoke("get_dialogs");
      localStorage.setItem("dialogs", JSON.stringify(dialogs));
    }
  });

  $: auth = false;
  $: dialogs = [];

  function switchEndpoint(ep) {
    endpoint.set(ep);
  }
</script>

<main class="container">
  {#if $endpoint == "/"}
    {#each dialogs as dlg}
      <div class="dialog">
        <img
          width="50px"
          height="50px"
          style="border-radius: 100%"
          src={`data:image/png;base64,${btoa(
            String.fromCharCode(...dlg.photo),
          )}`}
          alt="avatar"
        />
        <b>{dlg.name}</b>
      </div>
    {/each}

    {#if !auth}
      <button
        on:click={() => {
          switchEndpoint("/login");
        }}
      >
        Log in
      </button>
    {/if}
  {:else if $endpoint == "/login"}
    <Login />
  {:else}
    <h1>404</h1>
  {/if}
</main>

<style>
  main {
    padding: 1rem;
    background-color: #010521;
    color: #d4d8fd;
    font-family: "Montserrat", sans-serif;
  }

  .dialog {
    margin: 0rem 1rem 1rem 1rem;
    padding: 1rem;
    background-color: #8393fb;
    color: #010521;
  }
</style>
