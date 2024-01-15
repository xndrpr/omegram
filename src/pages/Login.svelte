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
  <h1>Login</h1>

  {#if step == 1}
    <b>Phone</b>
    <input bind:value={phone} placeholder="+XXxxxxxxx" />
    <button
      on:click={async () => {
        await invoke("request_code", { phone });

        nextStep();
      }}>Continue</button
    >
  {:else if step == 2}
    <b>Code</b>
    <input bind:value={code} placeholder="XXXXXXXX" />
    <b>{code}</b>
    <button
      on:click={async () => {
        await invoke("sign_in", { code });

        nextStep();

        endpoint.set("/");
      }}>Continue</button
    >
  {/if}
</main>

<style>
  main {
    text-align: center;
    vertical-align: middle;
  }
</style>
