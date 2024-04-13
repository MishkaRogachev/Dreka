import { type MissionRoute, type MissionRouteItem, MissionRouteItemType } from '$bindings/mission';

import { MapMissionRouteEvent, type MapMissionRoute } from '$lib/interfaces/map';
import { MapInteractionCesium } from '$lib/map/cesium/interaction';
import { BillboardEntity, PylonEntity, CircleEntity } from "$lib/map/cesium/base-entities"
import { cartesianFromGeodetic, geodeticFromCartesian } from '$lib/map/cesium/utils';

import * as Cesium from 'cesium';

import wptIcon from "$assets/svg/wpt.svg";
import takeoffIcon from "$assets/svg/takeoff.svg";
import landingIcon from "$assets/svg/landing.svg";

class MapMissionRouteItemCesium {
    constructor(route: MapMissionRouteCesium, cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        this.route = route;
        this.interaction = interaction;

        this.billboard = new BillboardEntity(cesium);

        this.billboard.setDraggable(true);
        this.billboard.subscribeDragging((cartesian: Cesium.Cartesian3) => { this.onPointDragging(cartesian) });
        this.billboard.subscribeDragged((cartesian: Cesium.Cartesian3) => { this.onPointDragged(cartesian) });
        this.billboard.subscribeClick(() => {
            if (this.item) {
                this.route.invoke(MapMissionRouteEvent.Activated, this.item, this.inRouteIndex());
            }
        });
        this.interaction.addInteractable(this.billboard);

        this.pylon = new PylonEntity(cesium, 4.0);

        this.circle = new CircleEntity(cesium, 4.0);
        // TODO: add interaction
        // this.circle.setDraggable(true)
        // this.circle.subscribeDragged((radius: number) => { this.onRadiusUpdated(radius) })
        // this.interaction.addInteractable(this.circle)
    }

    done() {
        this.interaction.removeInteractable(this.billboard);
        //this.interaction.removeInteractable(this.circle)
        this.billboard.done();
        this.pylon.done();
        this.circle.done();
        this.route.invoke(MapMissionRouteEvent.Removed, this.item!, this.inRouteIndex());
    }

    onPointDragging(cartesian: Cesium.Cartesian3) {
        const geodetic = geodeticFromCartesian(cartesian);
        if (this.item && geodetic) {
            this.item.position = geodetic;
            this.route.invoke(MapMissionRouteEvent.Drag, this.item, this.inRouteIndex());
        }
        this.pylon.setCartesian(this.billboard.cartesian());
        this.circle.setCartesian(this.billboard.cartesian());
    }

    onPointDragged(cartesian: Cesium.Cartesian3) {
        const geodetic = geodeticFromCartesian(cartesian);
        if (this.item && geodetic) {
            this.item.position = geodetic;
            this.route.invoke(MapMissionRouteEvent.Changed, this.item, this.inRouteIndex());
        }
    }

    update(item: MissionRouteItem) {
        this.item = item;
        // TODO: home altitude
        const cartesian = item.position ? cartesianFromGeodetic(item.position, 0) : Cesium.Cartesian3.ZERO;
        const loiterRadius = item.radius || 0;

        let icon: string;
        switch (item.type) {
        case MissionRouteItemType.Takeoff:
            icon = takeoffIcon;
            break;
        case "Landing":
            icon = landingIcon;
            break;
        default:
            icon = wptIcon;
            break;
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

    inRouteIndex(): number {
        return this.route.indexOfRouteItem(this);
    }

    private interaction: MapInteractionCesium;

    private billboard: BillboardEntity;
    private pylon: PylonEntity;
    private circle: CircleEntity;

    private route: MapMissionRouteCesium;
    private item: MissionRouteItem | undefined;
}

export class MapMissionRouteCesium implements MapMissionRoute {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        this.cesium = cesium;
        this.interaction = interaction;

        this.items = [];
        this.tracks = [];
        this.listeners = new Map();
    }

    done() {
        this.tracks.forEach(track => this.cesium.entities.remove(track))
        this.items.forEach(item => item.done());
        this.items = [];
    }

    subscribe(event: MapMissionRouteEvent, listener: (item: MissionRouteItem, index: number) => void) {
        this.listeners.set(event, listener);
    }

    invoke(event: MapMissionRouteEvent, item: MissionRouteItem, index: number) {
        let cb = this.listeners.get(event);
        if (cb) cb(item, index);
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
        this.items.push(new MapMissionRouteItemCesium(this, this.cesium, this.interaction));
        if (this.items.length > 1) {
            this.addTrack(this.items[this.items.length - 2], this.items[this.items.length - 1]);
        }
    }

    deleteRouteItem(index: number) {
        const hasRightBuddy = index + 1 < this.items.length;
        const hasLeftBuddy = index > 0;

        this.items[index].done();
        this.items.splice(index, 1);

        if (hasRightBuddy) {
            this.removeTrack(index);
        }
        if (hasLeftBuddy) {
            this.removeTrack(index - 1);
        }

        if (hasRightBuddy && hasLeftBuddy) {
            this.addTrack(this.items[index - 1], this.items[index], index - 1);
        }
    }

    addTrack(first: MapMissionRouteItemCesium, second: MapMissionRouteItemCesium, index = -1) {
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

    removeTrack(index: number) {
        this.cesium.entities.remove(this.tracks[index]);
        this.tracks.splice(index, 1);
    }

    indexOfRouteItem(item: MapMissionRouteItemCesium): number {
        return this.items.indexOf(item);
    }

    private items: Array<MapMissionRouteItemCesium>
    private tracks: Array<Cesium.Entity>

    private cesium: Cesium.Viewer
    private interaction: MapInteractionCesium

    private listeners: Map<MapMissionRouteEvent, (item: MissionRouteItem, index: number) => void>
}