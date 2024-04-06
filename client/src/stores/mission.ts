import { writable, derived, get } from 'svelte/store';

import type { Mission, MissionStatus } from '$bindings/mission';

import type { WsListener } from '$datasource/ws';
import { ClientSideEvents, EventsService } from '$services/events';
import { MissionService } from '$services/mission';
import { selectedVehicleID } from '$stores/vehicles';

export const missions = function () {
    let wsConnected: WsListener;

    const store = writable(new Map<string, Mission>(), (_, update) => {
        wsConnected = async (_data: any) => {
            let missions = await MissionService.getMissions();
            if (missions) {
                let missionsMap = new Map(missions!.map(mission => [mission.id, mission]));
                update(_ => { return missionsMap; });
            }
        }
    });

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        mission: (missionId: string) => get(store).get(missionId),
        missionIds: () => Array.from(get(store).keys()),
        missions: () => get(store).values(),
        download: async (missionId: string) => {
            await MissionService.downloadMission(missionId);
        },
        upload: async (missionId: string) => {
            await MissionService.uploadMission(missionId);
        },
        clear: async (missionId: string) => {
            await MissionService.clearMission(missionId);
        },
        kill: () => {
            EventsService.unsubscribe(ClientSideEvents.WsConnectionOpened, wsConnected);
        }
    }
} ()

export const selectedVehicleMission = derived([missions, selectedVehicleID], ($data) => {
    for (let mission of $data[0].values()) {
        if (mission.vehicle_id === $data[1]) {
            return mission;
        }
    }
    return undefined;
})
