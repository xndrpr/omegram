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
      dialogs = await invoke("get_dialogs");
    }
  });

  $: auth = false;
  $: dialogs = [];

  function switchEndpoint(ep) {
    endpoint.set(ep);
  }
</script>

<svelte:head>Omegram</svelte:head>

<main class="container">
  {#if $endpoint == "/"}
    <h1>Home</h1>

    {#each dialogs as dlg}
      <i>{dlg}</i>
      <hr />
      <br />
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
