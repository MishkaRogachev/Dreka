<script lang="ts">
import { clickOutside } from '$lib/common/click-outside';

import BaseModal from "$components/common/BaseModal.svelte";
import { i18n } from "$stores/i18n";

</script>

<style>
.max-scroll-area-height {
    max-height: 70vh;
}
</style>

<BaseModal id="about_modal">
    <form method="dialog">
        <!-- CLOSE -->
        <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
    </form>
    <h3 class="font-bold text-lg text-center mb-4">{ $i18n.t("About") }</h3>

    <!-- CONTENTS COMPONENT -->
    <div class="space-y-2 max-scroll-area-height overflow-y-auto">
        <!-- TODO: github -->
        <!-- TODO: discord -->
        <!-- TODO: mail -->
        <!-- TODO: description -->
    </div>
</BaseModal>
