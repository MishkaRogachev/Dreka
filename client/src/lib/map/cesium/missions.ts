import type { MissionRouteItem } from '$bindings/mission';

import { type MapMissions, type MapMissionsEvent, type MapMissionsEventListener } from '$lib/interfaces/map';
import { MapInteractionCesium } from '$lib/map/cesium/interaction';
import { MapMissionRouteCesium } from '$lib/map/cesium/mission_route';

import * as Cesium from 'cesium';

export class MapMissionsCesium implements MapMissions {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        this.cesium = cesium;
        this.interaction = interaction;

        this.selectedMissionId = "";
        this.missions = new Map();
        this.listeners = [];
    }

    done() {
        this.missions.forEach(mission => mission.done());
        this.missions.clear();
    }

    subscribe(listener: MapMissionsEventListener) {
        this.listeners.push(listener);
    }

    unsubscribe(listener: MapMissionsEventListener) {
        this.listeners = this.listeners.filter(l => l !== listener);
    }

    invoke(event: MapMissionsEvent) {
        this.listeners.forEach(listener => listener(event));
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
    private listeners: Array<MapMissionsEventListener>
}