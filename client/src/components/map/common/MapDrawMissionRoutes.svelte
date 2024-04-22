<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import { VehicleMode } from '$bindings/vehicles';
import type { Mission, MissionRouteItem } from '$bindings/mission';

import { vehicles } from '$stores/vehicles';
import { missions } from '$stores/mission';
import { activeMapPopup } from '$stores/app';
import { VehicleTelemetry, vehiclesTelemetry } from '$stores/telemetry';

import { MapMissionsEvent, type MapMissions, type MapViewport } from '$lib/interfaces/map';

import WaypointMenu from '$components/map/common/WaypointPopup.svelte';

export let viewport: MapViewport;
export let mapMissions: MapMissions;

let activatedItem: [MissionRouteItem, string, number] | undefined;

onMount(async () => {
    missions.subscribe((allMissions: Map<string, Mission>) => {
        let usedIds = new Array<string>();

        // Add and update existing missions on map
        allMissions.forEach((mission: Mission, missionId: string) => {
            usedIds.push(missionId);
            let mapMission = mapMissions.mission(missionId);
            if (!mapMission) {
                mapMission = mapMissions.addMission(missionId);
            }
            mapMission.updateFromRoute(mission.route);

            const inMission = vehicles.vehicle(mission.vehicle_id)?.status?.mode === VehicleMode.Mission;
            mapMission.updateFromProgress(mission.status.progress, inMission);
        });

        // Delete missions removed in store
        for (const missionId of mapMissions.missionIds()) {
            if (!usedIds.includes(missionId)) {
                mapMissions.removeMission(missionId)
            }
        }
    })

    vehiclesTelemetry.subscribe((tmi: Map<string, VehicleTelemetry>) => {
        tmi.forEach((tmi: VehicleTelemetry, vehicleId: string) => {
            for (const [missionId, mission] of $missions) {
                if (mission.vehicle_id === vehicleId) {
                    let mission = mapMissions.mission(missionId);
                    if (mission && tmi.navigation) {
                        mission.setHomeAltitude(tmi.navigation.home_position.altitude);
                    }
                }
            }
        });
    });

    mapMissions.subscribe(MapMissionsEvent.Changed, (missionId: string, item: MissionRouteItem, index: number) => {
        missions.setRouteItem(missionId, item, index);
    });
    mapMissions.subscribe(MapMissionsEvent.Activated, (missionId: string, item: MissionRouteItem, index: number) => {
        activatedItem = [item, missionId, index];
        $activeMapPopup = "waypoint";
    });
    mapMissions.subscribe(MapMissionsEvent.Removed, (missionId: string, _item: MissionRouteItem, index: number) => {
        if (activatedItem && activatedItem[1] === missionId && activatedItem[2] === index) {
            activatedItem = undefined;
        }
    });
})

onDestroy(async () => {
    mapMissions.done();
})

</script>

{#if activatedItem && $activeMapPopup === "waypoint" }
    <WaypointMenu viewport={viewport} routeItem={activatedItem[0]} missionId={activatedItem[1]} index={activatedItem[2]}
    on:close={() => { activatedItem = undefined; $activeMapPopup = "" }}/>
{/if}