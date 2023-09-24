<script lang="ts">
  import {ping} from "../lib/services/root"

  let pongMsg: String = ""

  async function greet() {
    pongMsg = await ping()
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <button type="submit">Ping</button>
  </form>
  <p>{pongMsg}</p>
</div>