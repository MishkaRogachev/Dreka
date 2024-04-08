import { type Mission, type MissionRoute, type MissionRouteItem } from '$bindings/mission';

import { MapInteractionCesium } from '$lib/map/cesium/interaction';
import { BillboardEntity, PylonEntity, CircleEntity } from "$lib/map/cesium/base-entities"
import { cartesianFromGeodetic } from '$lib/map/cesium/utils';

import * as Cesium from 'cesium';

import wptIcon from "$assets/svg/wpt.svg"
import takeoffIcon from "$assets/svg/takeoff.svg"
import landingIcon from "$assets/svg/landing.svg"

class MapMissionRouteItemCesium {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        this.cesium = cesium
        this.interaction = interaction

        this.billboard = new BillboardEntity(cesium)
        // TODO: add interaction
        // this.billboard.setDraggable(true)
        // this.billboard.subscribeDragging((cartesian: Cesium.Cartesian3) => { this.onPointDragging(cartesian) })
        // this.billboard.subscribeDragged((cartesian: Cesium.Cartesian3) => { this.onPointDragged(cartesian) })
        // this.billboard.subscribeClick(() => { UiDispatcher.instance().openRouteItemMenu(this) })
        // this.interaction.addInteractable(this.billboard)

        this.pylon = new PylonEntity(cesium, 4.0)

        this.circle = new CircleEntity(cesium, 4.0)
        // this.circle.setDraggable(true)
        // this.circle.subscribeDragged((radius: number) => { this.onRadiusUpdated(radius) })
        // this.interaction.addInteractable(this.circle)
    }

    done() {
        //this.interaction.removeInteractable(this.billboard)
        this.billboard.done()
        this.pylon.done()
        this.circle.done()
    }

    update(item: MissionRouteItem) {
        // TODO: to waypoint params
        let cartesian = Cesium.Cartesian3.ZERO;
        let loiterRadius = 0;
        let icon = wptIcon;

        // FIXME: flat serialization
        if (item.Takeoff) {
            cartesian = cartesianFromGeodetic(item.Takeoff.position, 0); // TODO: home altitude
            icon = takeoffIcon;
        }
        else if (item.Waypoint) {
            cartesian = cartesianFromGeodetic(item.Waypoint.position, 0); // TODO: home altitude
        }
        else if (item.LoiterTrn) {
            cartesian = cartesianFromGeodetic(item.LoiterTrn.position, 0); // TODO: home altitude
            loiterRadius = item.LoiterTrn.radius;
        }
        else if (item.LoiterAlt) {
            cartesian = cartesianFromGeodetic(item.LoiterAlt.position, 0); // TODO: home altitude
            loiterRadius = item.LoiterAlt.radius;
        }
        else if (item.Landing) {
            cartesian = cartesianFromGeodetic(item.Landing.position, 0); // TODO: home altitude
            icon = landingIcon;
        }

        const isCartesianValid = cartesian !== Cesium.Cartesian3.ZERO;

        this.billboard.setCartesian(cartesian);
        this.billboard.setIcon(icon);
        this.billboard.setVisible(isCartesianValid);

        this.pylon.setCartesian(cartesian);
        this.pylon.setVisible(isCartesianValid);

        this.circle.setCartesian(cartesian);
        this.circle.setRadius(loiterRadius);
        this.circle.setVisible(isCartesianValid && loiterRadius > 0);
    }

    private cesium: Cesium.Viewer;
    private interaction: MapInteractionCesium;

    private billboard: BillboardEntity;
    private pylon: PylonEntity;
    private circle: CircleEntity;
}

export class MapMissionRouteCesium {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        this.cesium = cesium;
        this.interaction = interaction;

        this.items = [];
    }

    done() {
        this.items.forEach(item => item.done());
        this.items = [];
    }

    update(route: MissionRoute) {
        // Remove extra items
        while (this.items.length > route.items.length) {
            this.items.pop()?.done();
        }
        // Add missing items
        while (this.items.length < route.items.length) {
            this.items.push(new MapMissionRouteItemCesium(this.cesium, this.interaction));
        }

        route.items.forEach((item, i) => {
            this.items[i].update(item);
        });
    }

    private items: Array<MapMissionRouteItemCesium>
    private cesium: Cesium.Viewer
    private interaction: MapInteractionCesium
}