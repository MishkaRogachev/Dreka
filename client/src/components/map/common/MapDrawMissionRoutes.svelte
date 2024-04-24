<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import { VehicleMode } from '$bindings/vehicles';
import type { Mission, MissionRouteItem } from '$bindings/mission';

import { vehicles } from '$stores/vehicles';
import { missions } from '$stores/mission';
import { activeMapPopup } from '$stores/app';
import { VehicleTelemetry, vehiclesTelemetry } from '$stores/telemetry';

import { type MapMissionsEvent, type MapMissions, type MapViewport } from '$lib/interfaces/map';

import WaypointMenu from '$components/map/common/WaypointPopup.svelte';

export let viewport: MapViewport;
export let mapMissions: MapMissions;

let selectedRouteItem: [MissionRouteItem, string, number] | undefined;

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

    mapMissions.subscribe((event: MapMissionsEvent) => {
        if (event.Activated) {
            selectedRouteItem = [event.Activated.item, event.Activated.missionId, event.Activated.index];
            $activeMapPopup = "waypoint_menu";
        } else if (event.ChangesOrdered) {
            missions.setRouteItem(event.ChangesOrdered.missionId, event.ChangesOrdered.item, event.ChangesOrdered.index);
        } else if (event.Hovered && $activeMapPopup !== "waypoint_menu") {
            selectedRouteItem = [event.Hovered.item, event.Hovered.missionId, event.Hovered.index];
            $activeMapPopup = "waypoint_tooltip";
        } else if (event.Exited) {
            selectedRouteItem = undefined;
            $activeMapPopup = "";
        }
    });
})

onDestroy(async () => {
    mapMissions.done();
})

</script>

{#if selectedRouteItem}
    <WaypointMenu viewport={viewport} routeItem={selectedRouteItem[0]} missionId={selectedRouteItem[1]} index={selectedRouteItem[2]}
    on:close={() => { selectedRouteItem = undefined; $activeMapPopup = "" }}/>
{/if}