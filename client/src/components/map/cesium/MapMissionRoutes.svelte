<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import * as Cesium from 'cesium';

import type { Mission, MissionRouteItem } from '$bindings/mission';
import { missions } from '$stores/mission';
import { activeMapPopup } from '$stores/app';

import { MapMissionRouteEvent, type MapViewport } from '$lib/interfaces/map';
import { MapMissionRouteCesium } from '$lib/map/cesium/mission_route';
import type { MapInteractionCesium } from '$lib/map/cesium/interaction';

import WaypointMenu from '$components/map/common/WaypointPopup.svelte';

export let cesium: Cesium.Viewer;
export let interaction: MapInteractionCesium;
export let viewport: MapViewport;

let mapRouteItems = new Map<string, MapMissionRouteCesium>

let activatedItem: [MissionRouteItem, string, number] | undefined;

onMount(async () => {
    missions.subscribe((allMissions: Map<string, Mission>) => {
        let usedIds = new Array<string>();

        // Add and update existing missions on map
        allMissions.forEach((mission: Mission, missionID: string) => {
            usedIds.push(missionID);
            if (!mapRouteItems.has(missionID)) {
                let mapMission = new MapMissionRouteCesium(cesium, interaction)
                mapMission.subscribe(MapMissionRouteEvent.Changed, (item: MissionRouteItem, index: number) => {
                    missions.setRouteItem(missionID, item, index);
                });
                mapMission.subscribe(MapMissionRouteEvent.Activated, (item: MissionRouteItem, index: number) => {
                    activatedItem = [item, missionID, index];
                    $activeMapPopup = "waypoint";
                });
                mapMission.subscribe(MapMissionRouteEvent.Drag, (_item: MissionRouteItem, _index: number) => {
                    activatedItem = undefined;
                });
                mapMission.subscribe(MapMissionRouteEvent.Removed, (_item: MissionRouteItem, index: number) => {
                    if (activatedItem && activatedItem[1] === missionID && activatedItem[2] === index) {
                        activatedItem = undefined;
                    }
                });
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
})

onDestroy(async () => {
    for (const id of mapRouteItems.keys()) {
        mapRouteItems.get(id)?.done();
        mapRouteItems.delete(id);
    }
})

</script>

{#if activatedItem && $activeMapPopup === "waypoint" }
    <WaypointMenu viewport={viewport} routeItem={activatedItem[0]} missionId={activatedItem[1]} index={activatedItem[2]}
    on:close={() => { activatedItem = undefined; $activeMapPopup = "" }}/>
{/if}