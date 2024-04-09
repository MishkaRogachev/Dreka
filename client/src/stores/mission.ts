import { writable, derived, get } from 'svelte/store';

import type { Mission, MissionRoute, MissionRouteItem, MissionStatus } from '$bindings/mission';

import type { WsListener } from '$datasource/ws';
import { ClientSideEvents, EventsService } from '$services/events';
import { MissionService } from '$services/mission';
import { selectedVehicleID } from '$stores/vehicles';

export const missions = function () {
    let missionUpserted: WsListener;
    let missionRemoved: WsListener;
    let missionStatusUpdated: WsListener;
    let missionRouteUpdated: WsListener;
    let missionRouteItemUpserted: WsListener;
    let missionRouteItemRemoved: WsListener;

    let wsConnected: WsListener;

    const store = writable(new Map<string, Mission>(), (_, update) => {
        missionUpserted = (data: any) => {
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
        missionRouteUpdated = (data: any) => {
            let route = data["route"] as MissionRoute;
            if (!route) {
                return;
            }

            update(missions => {
                let mission = missions.get(route.id);
                if (mission) {
                    mission.route = route;
                }
                return missions;
            });
        }
        missionRouteItemUpserted = (data: any) => {
            let mission_id = data["mission_id"] as string;
            let index = data["index"] as number;
            let item = data["item"] as MissionRouteItem;

            if (!mission_id || index === undefined || !item) {
                return;
            }

            update(missions => {
                let mission = missions.get(mission_id);
                if (mission) {
                    if (index < mission.route.items.length) {
                        mission.route.items[index] = item;
                    } else {
                        mission.route.items.push(item);
                    }
                }
                return missions;
            });
        }
        missionRouteItemRemoved = (data: any) => {
            let mission_id = data["mission_id"] as string;
            let index = data["index"] as number;

            if (!mission_id || index === undefined) {
                return;
            }

            update(missions => {
                let mission = missions.get(mission_id);
                if (mission) {
                    mission.route.items.splice(index, 1);
                }
                return missions;
            });
        }
        wsConnected = async (_data: any) => {
            let missions = await MissionService.getMissions();
            if (missions) {
                let missionsMap = new Map(missions!.map(mission => [mission.id, mission]));
                update(_ => { return missionsMap; });
            }
        }

        EventsService.subscribe("MissionUpserted", missionUpserted);
        EventsService.subscribe("MissionRemoved", missionRemoved);
        EventsService.subscribe("MissionStatusUpdated", missionStatusUpdated);
        EventsService.subscribe("MissionRouteUpdated", missionRouteUpdated);
        EventsService.subscribe("MissionRouteItemUpserted", missionRouteItemUpserted);
        EventsService.subscribe("MissionRouteItemRemoved", missionRouteItemRemoved);
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
        setRouteItem: async (missionId: string, item: MissionRouteItem, index: number) => {
            let result = await MissionService.setRouteItem(missionId, item, index);
            let missions = get(store);
            let mission = missions.get(missionId);
            if (result && mission) {
                for (const [index, item] of result) {
                    if (index < mission.route.items.length) {
                        mission.route.items[index] = item;
                    } else {
                        mission.route.items.push(item);
                    }
                }
                missions.set(missionId, mission);
                store.update(_ => { return missions; });
            }
        },
        removeRouteItem: async (missionId: string, index: number) => {
            await MissionService.removeRouteItem(missionId, index);
        },
        download: async (missionId: string) => {
            await MissionService.downloadMission(missionId);
        },
        upload: async (missionId: string) => {
            await MissionService.uploadMission(missionId);
        },
        clear: async (missionId: string) => {
            let mission = await MissionService.clearMission(missionId);
            if (mission) {
                let missions = get(store);
                missions.set(mission.id, mission);
                store.update(_ => { return missions; });
            }
        },
        cancelState: async (missionId: string) => {
            await MissionService.cancelMissionState(missionId);
        },
        kill: () => {
            EventsService.unsubscribe("MissionUpserted", missionUpserted);
            EventsService.unsubscribe("MissionRemoved", missionRemoved);
            EventsService.unsubscribe("MissionStatusUpdated", missionStatusUpdated);
            EventsService.unsubscribe("MissionRouteUpdated", missionRouteUpdated);
            EventsService.unsubscribe("MissionRouteItemUpserted", missionRouteItemUpserted);
            EventsService.unsubscribe("MissionRouteItemRemoved", missionRouteItemRemoved);
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
