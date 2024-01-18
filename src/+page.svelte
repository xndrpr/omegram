<script>
  import { onMount } from "svelte";
  import Login from "./pages/Login.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { endpoint } from "./lib/store";
  import { cachedAuth, cachedDialogs } from "./lib/cache";

  let auth = false;
  let dialogs = [];

  onMount(async () => {
    auth = cachedAuth == true ? true : await invoke("check_auth");
    localStorage.setItem("auth", auth ? "true" : "false");

    if (auth == true) {
      dialogs =
        cachedDialogs.length > 0 ? cachedDialogs : await invoke("get_dialogs");
      localStorage.setItem("dialogs", JSON.stringify(dialogs));
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
      <div>
        <button
          class={auth ? "logout-btn" : "login-btn"}
          on:click={async () => {
            auth = !auth;
            localStorage.setItem("auth", auth ? "true" : "false");

            if (auth == true) {
              switchEndpoint("/login");
            } else {
              await invoke("logout");
            }
          }}
          >{#if auth}
            Log Out
          {:else}
            Log In
          {/if}</button
        >
      </div>
    </div>
    {#if auth}
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
    min-height: calc(100vh);
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

  .menu-item button {
    padding: 1rem;
    border: none;
    outline: none;
    background-color: rgba(255, 255, 255, 0.1);
    color: #fff;
    border-radius: 1rem;
    transition: all 0.3s ease;
  }

  .menu-item button:hover {
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

  .logout-btn {
    position: absolute;
    bottom: 0;
    left: 0;
    margin-left: 1.6rem;
    margin-bottom: 2rem;
    padding: 1rem;
    border-radius: 1rem;
    border: none;
    outline: none;
    background-color: #ff878793;
    color: #fff;

    transition: all 0.2s ease;
  }

  .logout-btn:hover {
    background-color: #ff8787;
  }

  .login-btn {
    position: absolute;
    bottom: 0;
    left: 0;
    margin-left: 1.6rem;
    margin-bottom: 2rem;
    padding: 1rem;
    border-radius: 1rem;
    border: none;
    outline: none;
    background-color: #8b87ff93;
    color: #fff;

    transition: all 0.2s ease;
  }

  .login-btn:hover {
    background-color: #af87ff;
  }
</style>
