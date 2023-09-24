<script lang="ts">
  let pongMsg = ""

  async function greet() {
    // TODO: ping server to get response
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <button type="submit">Ping</button>
  </form>
  <p>{pongMsg}</p>
</div>