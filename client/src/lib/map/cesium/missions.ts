import type { MissionRouteItem } from '$bindings/mission';

import { type MapMissions, MapMissionsEvent, type MapMissionsEventListener } from '$lib/interfaces/map';
import { MapInteractionCesium } from '$lib/map/cesium/interaction';
import { MapMissionRouteCesium } from '$lib/map/cesium/mission_route';

import * as Cesium from 'cesium';

export class MapMissionsCesium implements MapMissions {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        this.cesium = cesium;
        this.interaction = interaction;

        this.selectedMissionId = "";
        this.missions = new Map();
        this.listeners = new Map();
    }

    done() {
        this.listeners.clear();
    }

    subscribe(event: MapMissionsEvent, listener: MapMissionsEventListener) {
        this.listeners.set(event, listener);
    }

    invoke(event: MapMissionsEvent, missionId: string, item: MissionRouteItem, index: number) {
        let cb = this.listeners.get(event);
        if (cb) cb(missionId, item, index);
    }

    setSelectedMission(missionId: string) {
        this.selectedMissionId = missionId;
        // TODO: implement
    }

    addMission(missionId: string) {
        let mission = new MapMissionRouteCesium(missionId, this);
        this.missions.set(missionId, mission);
        return mission;
    }

    removeMission(missionId: string) {
        this.missions.get(missionId)?.done();
        this.missions.delete(missionId);
    }

    mission(missionId: string) {
        return this.missions.get(missionId);
    }

    allMissions() {
        return Array.from(this.missions.values());
    }

    missionIds() {
        return Array.from(this.missions.keys());
    }

    cesium: Cesium.Viewer
    interaction: MapInteractionCesium

    selectedMissionId: string;
    private missions: Map<string, MapMissionRouteCesium>
    private listeners: Map<MapMissionsEvent, MapMissionsEventListener>
}