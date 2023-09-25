<script lang="ts">
    import { AppService } from "$lib/services/app";

    let pongMsg: String = "";
    let appService = new AppService();

    async function greet() {
        appService.pingServer();
    }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <button type="submit">Ping</button>
  </form>
  <p>{pongMsg}</p>
</div>