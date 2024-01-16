<script>
  import { onMount } from "svelte";
  import Login from "./pages/Login.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { endpoint } from "./lib/store";

  let auth = false;
  let dialogs = [];

  onMount(async () => {
    auth = true;
    if (auth == false) {
      switchEndpoint("/login");
    } else {
      dialogs = JSON.parse(localStorage.getItem("dialogs"));
    }
  });

  function switchEndpoint(ep) {
    endpoint.set(ep);
  }
</script>

<main class="container">
  {#if $endpoint == "/"}
    <div class="side-menu">
      <div class="menu-item">
        <button>Chats</button>
      </div>
    </div>
    <div class="dialogs">
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
    </div>

    <div class="chat">
      <i>Select a chat</i>
    </div>

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
    overflow: hidden;
    padding: 1rem;
    background-color: rgba(1, 5, 33, 0.8);
    color: #d4d8fd;
    font-family: "Montserrat", sans-serif;
    backdrop-filter: blur(10px);
  }

  .side-menu {
    padding: 1rem;
    border-radius: 1rem;
    background-color: rgba(1, 5, 33, 0.1);
    max-height: calc(100vh - 2rem);
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    gap: 1rem;
  }

  .side-menu button {
    padding: 1rem;
    border: none;
    outline: none;
    background-color: rgba(255, 255, 255, 0.1);
    color: #fff;
    border-radius: 1rem;
    transition: all 0.3s ease;
  }

  .side-menu button:hover {
    background-color: rgba(255, 255, 255, 0.2);
  }

  .container {
    display: flex;
  }

  .dialogs {
    overflow-y: auto;
    max-height: calc(100vh - 2rem);
  }

  .chat {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    padding: 1rem;
    border-radius: 1rem;
    background-color: rgba(255, 255, 255, 0.1);
  }

  .dialog {
    position: relative;
    width: auto;
    display: flex;
    justify-content: flex-start;
    gap: 0.5rem;
    align-items: center;
    margin: 0rem 1rem 1rem 1rem;
    padding: 1rem;
    border-radius: 15px;
    overflow: hidden;
    color: #f8f8f8;
    position: relative;
    background-color: rgba(255, 255, 255, 0.1);

    transition: all 0.3s ease;
  }

  .dialog:hover {
    background-color: rgba(255, 255, 255, 0.2);
  }
</style>
