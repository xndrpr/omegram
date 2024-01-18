<script>
  import { invoke } from "@tauri-apps/api";
  import { endpoint } from "../lib/store";

  $: step = 1;

  $: code = "";
  $: phone = "";

  function nextStep() {
    step += 1;
  }
</script>

<main>
  {#if step == 1}
    <div class="form">
      <b>Phone</b>
      <input bind:value={phone} placeholder="+XXxxxxxxx" />
      <button
        on:click={async () => {
          nextStep();

          await invoke("request_code", { phone });
        }}>Continue</button
      >
    </div>
  {:else if step == 2}
    <div class="form">
      <b>Code</b>
      <input bind:value={code} placeholder="XXXXXXXX" />
      <button
        on:click={async () => {
          await invoke("sign_in", { code });
          localStorage.setItem("auth", "true");

          nextStep();
          endpoint.set("/");
        }}>Continue</button
      >
    </div>
  {/if}
</main>

<style>
  main {
    gap: 1rem;
    min-width: calc(100vw);
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
  }
  .form {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    gap: 0.5rem;
  }

  button {
    border-radius: 1rem;
    padding: 0.5rem 1rem;
    border: none;
    outline: none;

    background-color: #b49be8;

    transition: all 0.2s ease;
  }

  button:hover {
    padding: 1rem;
  }

  input {
    font-size: 1rem;
    border-radius: 1rem;
    padding: 0.5rem;
    outline: none;
    border: none;

    background-color: #b49be8;
    color: #eee;
  }

  input::placeholder {
    color: #ddd;
  }
</style>
