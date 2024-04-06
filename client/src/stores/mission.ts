import { writable, derived, get } from 'svelte/store';

import type { Mission, MissionStatus } from '$bindings/mission';

import type { WsListener } from '$datasource/ws';
import { ClientSideEvents, EventsService } from '$services/events';
import { MissionService } from '$services/mission';
import { selectedVehicleID } from '$stores/vehicles';

export const missions = function () {
    let missionUpdated: WsListener;
    let missionRemoved: WsListener;
    let missionStatusUpdated: WsListener;
    let wsConnected: WsListener;

    const store = writable(new Map<string, Mission>(), (_, update) => {
        missionUpdated = (data: any) => {
            let mission = data["mission"] as Mission;
            if (!mission) {
                return;
            }

            update(missions => {
                missions.set(mission.id, mission);
                return missions;
            });
        }
        missionRemoved = (data: any) => {
            let id = data["mission_id"] as string;
            if (!id) {
                return;
            }

            update(missions => {
                if (missions.has(id)) {
                    missions.delete(id);
                }
                return missions;
            });
        }
        missionStatusUpdated = (data: any) => {
            let status = data["status"] as MissionStatus;
            if (!status) {
                return;
            }

            update(missions => {
                let mission = missions.get(status.id);
                if (mission) {
                    mission.status = status;
                }
                return missions;
            });
        }

        wsConnected = async (_data: any) => {
            let missions = await MissionService.getMissions();
            console.log('missions', missions);
            if (missions) {
                let missionsMap = new Map(missions!.map(mission => [mission.id, mission]));
                update(_ => { return missionsMap; });
            }
        }

        EventsService.subscribe("MissionUpdated", missionUpdated);
        EventsService.subscribe("MissionRemoved", missionRemoved);
        EventsService.subscribe("MissionStatusUpdated", missionStatusUpdated);
        EventsService.subscribe(ClientSideEvents.WsConnectionOpened, wsConnected);
    });

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        mission: (missionId: string) => get(store).get(missionId),
        missionIds: () => Array.from(get(store).keys()),
        missions: () => get(store).values(),
        createVehicleMission: async (vehicleId: string) => {
            let mission = await MissionService.createMission(vehicleId);
            if (mission) {
                let missions = get(store);
                missions.set(mission.id, mission);
                store.update(_ => { return missions; });
            }
        },
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
            EventsService.unsubscribe("MissionUpdated", missionUpdated);
            EventsService.unsubscribe("MissionRemoved", missionRemoved);
            EventsService.unsubscribe("MissionStatusUpdated", missionStatusUpdated);
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
