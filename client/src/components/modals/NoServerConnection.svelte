<script lang="ts">
import { i18n } from "$stores/i18n";
</script>

<div id="modal" class="fixed inset-0 z-50 flex justify-center items-center bg-black bg-opacity-50">
    <div id="banner" class="bg-base-300 p-8 rounded-lg text-center">
        <h3 class="font-bold text-lg mb-4">{ $i18n.t("NO SERVER CONNECTION") }</h3>
        <div class="flex items-center space-x-2">
            <span class="loading loading-dots loading-lg"></span>
            <p>{ $i18n.t("Connecting to brygge...") }</p>
        </div>
    </div>
</div>
