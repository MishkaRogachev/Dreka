<script lang="ts">
import { onMount, onDestroy } from 'svelte';
import { get } from 'svelte/store';

import * as Cesium from 'cesium';

import type { Mission } from '$bindings/mission';

import { MapMissionRouteCesium } from '$lib/map/cesium/mission_route';
import type { MapInteractionCesium } from '$lib/map/cesium/interaction';

import { missions } from '$stores/mission';

export let cesium: Cesium.Viewer;
export let interaction: MapInteractionCesium;

let mapRouteItems = new Map<string, MapMissionRouteCesium>

onMount(async () => {
    missions.subscribe((missions: Map<string, Mission>) => {
        let usedIds = new Array<string>();

        // Add and update existing missions on map
        missions.forEach((mission: Mission, missionID: string) => {
            usedIds.push(missionID);
            if (!mapRouteItems.has(missionID)) {
                let mapMission = new MapMissionRouteCesium(cesium, interaction)
                // mapMission.setSelected(missionID === get(selectedMissionID));
                mapRouteItems.set(missionID, mapMission);
            }
            mapRouteItems.get(missionID)!.update(mission.route);
        });

        // Delete missions removed in store
        for (const id of mapRouteItems.keys()) {
            if (!usedIds.includes(id)) {
                mapRouteItems.get(id)?.done();
                mapRouteItems.delete(id);
            }
        }
    })

    // selectedMissionID.subscribe((selectedMissionID: string) => {
    //     mapRouteItems.forEach((mission: MapMissionCesium, missionID: string) => {
    //         mission.setSelected(missionID === selectedMissionID);
    //     });
    // });
})

onDestroy(async () => {
    for (const id of mapRouteItems.keys()) {
        mapRouteItems.get(id)?.done();
        mapRouteItems.delete(id);
    }
})

</script>
