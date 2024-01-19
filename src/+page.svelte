<script>
  import { afterUpdate, onMount } from "svelte";
  import Login from "./pages/Login.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { auth, dialogs, endpoint, messages } from "./lib/store";

  let msgs;

  afterUpdate(() => {
    console.log("afterUpdate");
    if (msgs) scrollToBottom(msgs);
  });

  $: if (msgs) {
    console.log("tick");
    scrollToBottom(msgs);
  }

  const scrollToBottom = async (node) => {
    console.log("Scroll Height Before:", node.scrollHeight);
    node.scroll({ top: node.scrollHeight, behavior: "smooth" });
    console.log("Scroll Height After:", node.scrollHeight);
  };

  onMount(async () => {
    console.log($messages);
    auth.set($auth == true ? true : await invoke("check_auth"));
    localStorage.setItem("auth", auth ? "true" : "false");

    if ($auth == true) {
      localStorage.setItem("dialogs", JSON.stringify($dialogs));
      let xxx = await invoke("get_dialogs");
      dialogs.set(xxx);

      messages.set(JSON.parse(localStorage.getItem("messages")));
    }
  });

  function switchEndpoint(ep) {
    endpoint.set(ep);
  }

  async function logout() {
    auth.set(!$auth);
    localStorage.setItem("auth", auth ? "true" : "false");

    if ($auth == true) {
      switchEndpoint("/login");
    } else {
      await invoke("logout");
    }
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
          class={$auth ? "logout-btn" : "login-btn"}
          on:click={async () => {
            await logout();
            console.log("HEY");
          }}
          >{#if $auth}
            Log Out
          {:else}
            Log In
          {/if}</button
        >
      </div>
    </div>
    <div class="dialogs">
      {#each $dialogs as dlg}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div
          class="dialog"
          id={dlg.id}
          on:click={async () => {
            messages.set(await invoke("get_messages", { id: dlg.id }));
            // localStorage.setItem("messages", JSON.stringify(messages));
          }}
        >
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
      {#if $messages.length <= 0}
        <i>Select a chat</i>
      {:else}
        <div class="messages" bind:this={msgs}>
          {#if $messages.length > 0}
            {#each $messages as msg}
              <div class="msg">
                {msg}
              </div>
            {/each}{/if}
        </div>
      {/if}
    </div>
  {:else if $endpoint == "/login"}
    <Login />
  {:else}
    <h1>404</h1>
  {/if}
</main>

<style>
  main {
    flex: 1;
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
    min-width: fit-content;
    max-height: calc(100vh - 2rem);
  }

  .chat {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 1rem;
    border-radius: 1rem;
    background-color: rgba(255, 255, 255, 0.1);
    max-height: calc(100vh - 2rem);
    overflow-y: auto;
  }

  .messages {
    display: flex;
    flex-direction: column;
  }

  .msg {
    width: fit-content;
    max-width: 90%;
    padding: 1rem;
    margin: 0.5rem 1rem;
    border-radius: 1rem;
    background-color: rgba(255, 255, 255, 0.1);
    white-space: pre-wrap;
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
