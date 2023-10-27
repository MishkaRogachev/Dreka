<script lang="ts">
export let id: string
</script>

<style>
.base-modal {
    max-height: 70vh;
    min-height: 40vh;
}
</style>

<dialog id={id} class="modal">
    <div class="modal-box base-modal w-11/12 max-w-5xl overflow-hidden flex flex-col">
        <slot />
    </div>
    <form method="dialog" class="modal-backdrop">
        <button>close</button>
    </form>
</dialog>