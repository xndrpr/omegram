<script>
  import { afterUpdate, onMount } from "svelte";
  import Login from "./pages/Login.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { auth, dialogs, endpoint, messages } from "./lib/store";

  $: isLoadingMessages = false;
  let chat;

  const scrollToBottom = async (node) => {
    node.scroll({ top: node.scrollHeight, behavior: "smooth" });
  };

  const handleScroll = async () => {
    if (chat.scrollTop === 0 && !isLoadingMessages) {
      console.log("Reached top of chat, fetch older messages");
      // wait for 2 seconds
      await new Promise((resolve) => setTimeout(resolve, 2000));
      isLoadingMessages = true;
      // TODO: Implement logic to fetch older messages
    }
  };

  onMount(async () => {
    if (chat) {
      chat.addEventListener("scroll", handleScroll);
    }

    auth.set($auth == true ? true : await invoke("check_auth"));

    await invoke("set_setting", {
      key: "auth",
      value: auth ? "true" : "false",
    });

    if ($auth == true) {
      let needed = await invoke("update_dialogs");
      console.log(needed);
      if (needed) {
        // let dlgs = await invoke("get_dialogs");
        let dlgs = JSON.parse(localStorage.getItem("dialogs"));
        dialogs.set(dlgs);
        console.log(dialogs);
        await invoke("set_setting", {
          key: "dialogs",
          value: JSON.stringify(dlgs),
        });
      }

      messages.set(
        JSON.parse(await invoke("get_setting", { key: "messages" })),
      );
    }
  });

  function switchEndpoint(ep) {
    endpoint.set(ep);
  }

  function bytesToPhoto(bytes) {
    return `data:image/png;base64,${btoa(String.fromCharCode(...bytes))}`;
  }

  async function logout() {
    auth.set(!$auth);
    await invoke("set_setting", {
      key: "auth",
      value: auth ? "false" : "true",
    });

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
          }}
          >{#if $auth}
            Log Out
          {:else}
            Log In
          {/if}</button
        >
      </div>
    </div>
    {#if $auth}
      <div class="dialogs">
        {#if $dialogs.length > 0}
          {#each $dialogs as dlg}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div
              class="dialog"
              id={dlg.id}
              on:click={async () => {
                messages.set(await invoke("get_messages", { id: dlg.id }));

                console.log($messages);
                setTimeout(() => {
                  try {
                    scrollToBottom(chat);
                  } catch {}
                });
              }}
            >
              <img
                width="50px"
                height="50px"
                style="border-radius: 100%"
                src={bytesToPhoto(dlg.photo)}
                alt="avatar"
              />
              <b>{dlg.name}</b>
            </div>
          {/each}
        {/if}
      </div>

      <div class="chat" bind:this={chat}>
        {#if $messages.length <= 0}
          <i>Select a chat</i>
        {:else}
          <div class="messages">
            {#if $messages.length > 0}
              {#each $messages as msg}
                <div class="msg">
                  <div class="avatar">
                    <img
                      width="50px"
                      height="50px"
                      style="border-radius: 100%"
                      src={bytesToPhoto(msg.avatar)}
                      alt="avatar"
                    />
                  </div>
                  <div class="text">
                    {msg.text}
                  </div>
                </div>
              {/each}{/if}
          </div>
        {/if}
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
    flex-grow: 1;
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
    display: flex;
  }

  .msg .text {
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
