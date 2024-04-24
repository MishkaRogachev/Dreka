<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import type { Geodetic } from '$bindings/spatial';
import { VehicleMode } from '$bindings/vehicles';
import type { Mission, MissionRouteItem } from '$bindings/mission';

import { vehicles } from '$stores/vehicles';
import { missions } from '$stores/mission';
import { activeMapPopup } from '$stores/app';
import { VehicleTelemetry, vehiclesTelemetry } from '$stores/telemetry';

import { type MapMissionsEvent, type MapFacade } from '$lib/interfaces/map';

import WaypointMenu from '$components/map/common/WaypointPopup.svelte';

export let map: MapFacade;

let selectedRouteItem: [MissionRouteItem, string, number] | undefined;
let overridedPosition: Geodetic | undefined;

onMount(async () => {
    missions.subscribe((allMissions: Map<string, Mission>) => {
        let usedIds = new Array<string>();

        // Add and update existing missions on map
        allMissions.forEach((mission: Mission, missionId: string) => {
            usedIds.push(missionId);
            let mapMission = map.missions.mission(missionId);
            if (!mapMission) {
                mapMission = map.missions.addMission(missionId);
            }
            mapMission.updateFromRoute(mission.route);

            const inMission = vehicles.vehicle(mission.vehicle_id)?.status?.mode === VehicleMode.Mission;
            mapMission.updateFromProgress(mission.status.progress, inMission);
        });

        // Delete missions removed in store
        for (const missionId of map.missions.missionIds()) {
            if (!usedIds.includes(missionId)) {
                map.missions.removeMission(missionId)
            }
        }
    })

    vehiclesTelemetry.subscribe((tmi: Map<string, VehicleTelemetry>) => {
        tmi.forEach((tmi: VehicleTelemetry, vehicleId: string) => {
            for (const [missionId, mission] of $missions) {
                if (mission.vehicle_id === vehicleId) {
                    let mission = map.missions.mission(missionId);
                    if (mission && tmi.navigation) {
                        mission.setHomeAltitude(tmi.navigation.home_position.altitude);
                    }
                }
            }
        });
    });

    map.missions.subscribe((event: MapMissionsEvent) => {
        if (event.InvokeWaypointMenu) {
            selectedRouteItem = [event.InvokeWaypointMenu.item, event.InvokeWaypointMenu.missionId, event.InvokeWaypointMenu.index];
            overridedPosition = undefined;
            $activeMapPopup = "waypoint_menu";
        } else if (event.ChangesOrdered) {
            missions.setRouteItem(event.ChangesOrdered.missionId, event.ChangesOrdered.item, event.ChangesOrdered.index);
            overridedPosition = undefined;
        } else if (event.Hovered && $activeMapPopup !== "waypoint_menu") {
            selectedRouteItem = [event.Hovered.item, event.Hovered.missionId, event.Hovered.index];
            overridedPosition = undefined;
            $activeMapPopup = "waypoint_tooltip";
        } else if (event.Exited) {
            if ($activeMapPopup !== "waypoint_menu") {
                selectedRouteItem = undefined;
                $activeMapPopup = "";
            }
            overridedPosition = undefined;
        } else if (event.WaypointDragged) {
            selectedRouteItem = [event.WaypointDragged.item, event.WaypointDragged.missionId, event.WaypointDragged.index];
            overridedPosition = event.WaypointDragged.position;
            $activeMapPopup = "waypoint_tooltip"
        }
    });
})

</script>

{#if selectedRouteItem}
    <WaypointMenu
        map={map}
        routeItem={selectedRouteItem[0]}
        missionId={selectedRouteItem[1]}
        index={selectedRouteItem[2]}
        overridedPosition={overridedPosition}
        on:close={() => { selectedRouteItem = undefined; $activeMapPopup = "" }}/>
{/if}
