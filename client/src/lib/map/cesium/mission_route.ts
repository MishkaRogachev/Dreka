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

        this.billboard.setCartesian(cartesian);
        this.billboard.setIcon(icon);

        this.pylon.setCartesian(cartesian);

        this.circle.setCartesian(cartesian);
        this.circle.setRadius(loiterRadius);
        this.circle.setVisible(loiterRadius > 0);
    }

    cartesian(): Cesium.Cartesian3 {
        return this.billboard.cartesian()
    }

    isPositionValid(): boolean {
        return !this.billboard.cartesian().equals(Cesium.Cartesian3.ZERO)
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
        this.tracks = [];
    }

    done() {
        this.items.forEach(item => item.done());
        this.items = [];
    }

    update(route: MissionRoute) {
        // Remove extra items
        while (this.items.length > route.items.length) {
            this.deleteRouteItem(this.items.length - 1);
        }
        // Add missing items
        while (this.items.length < route.items.length) {
            this.addRouteItem()
        }

        route.items.forEach((item, i) => {
            this.items[i].update(item);
        });
    }

    addRouteItem() {
        this.items.push(new MapMissionRouteItemCesium(this.cesium, this.interaction));
        if (this.items.length > 1) {
            this.addLine(this.items[this.items.length - 2], this.items[this.items.length - 1]);
        }
    }

    deleteRouteItem(index: number) {
        const hasRightBuddy = index + 1 < this.items.length;
        const hasLeftBuddy = index > 0;

        this.items.splice(index, 1)[0].done()

        if (hasRightBuddy) {
            this.removeLine(index);
        }
        if (hasLeftBuddy) {
            this.removeLine(index - 1);
        }

        if (hasRightBuddy && hasLeftBuddy) {
            this.addLine(this.items[index - 1], this.items[index], index - 1);
        }
    }

    addLine(first: MapMissionRouteItemCesium, second: MapMissionRouteItemCesium, index = -1) {
        const line = this.cesium.entities.add({
            polyline: {
                positions: new Cesium.CallbackProperty(() => {
                    if (first.isPositionValid() && second.isPositionValid())
                        return [first.cartesian(), second.cartesian()]
                    return []
                }, false),
                arcType: Cesium.ArcType.GEODESIC,
                //@ts-ignore
                material: new Cesium.ColorMaterialProperty(
                    new Cesium.CallbackProperty(() => {
                        return Cesium.Color.WHITE;
                        // const error = first.isLowerTerrrain() || second.isLowerTerrrain()
                        // const warn = first.isLowerWarningAlt() || second.isLowerWarningAlt()

                        // return error ? Cesium.Color.RED : warn ? Cesium.Color.ORANGE :
                        //     first.actual && second.actual ? Cesium.Color.WHITE : Cesium.Color.YELLOW
                    }, false),
                ),
                width: 2.0
            }
        })

        if (index === -1) {
            this.tracks.push(line);
        } else {
            this.tracks.splice(index, 0, line);
        }
    }

    removeLine(index: number) {
        this.cesium.entities.remove(this.tracks[index]);
        this.tracks.splice(index, 1);
    }

    private items: Array<MapMissionRouteItemCesium>
    private tracks: Array<Cesium.Entity>

    private cesium: Cesium.Viewer
    private interaction: MapInteractionCesium
}