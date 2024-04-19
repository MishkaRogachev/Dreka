import { type MissionRoute, type MissionRouteItem, MissionRouteItemType, type MissionProgress } from '$bindings/mission';

import { MapMissionsEvent, type MapMissionRoute } from '$lib/interfaces/map';
import { MapInteractionCesium } from '$lib/map/cesium/interaction';
import { MapMissionsCesium } from '$lib/map/cesium/missions';
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
                this.route.invoke(MapMissionsEvent.Activated, this.item, this.inRouteIndex());
            }
        });
        this.interaction.addInteractable(this.billboard);

        this.pylon = new PylonEntity(cesium, 4.0);
        this.pylon.setBaseColor(Cesium.Color.LIGHTSTEELBLUE);

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
        this.route.invoke(MapMissionsEvent.Removed, this.item!, this.inRouteIndex());
    }

    onPointDragging(cartesian: Cesium.Cartesian3) {
        if (!this.item?.position) {
            return;
        }
        const geodetic = geodeticFromCartesian(cartesian, this.item!.position!.frame, this.route.homeAltitude);
        if (geodetic) {
            this.item.position = geodetic;
            this.route.invoke(MapMissionsEvent.Drag, this.item, this.inRouteIndex());
        }
        this.pylon.setCartesian(this.billboard.cartesian());
        this.circle.setCartesian(this.billboard.cartesian());
    }

    onPointDragged(cartesian: Cesium.Cartesian3) {
        if (!this.item?.position) {
            return;
        }
        const geodetic = geodeticFromCartesian(cartesian, this.item!.position!.frame, this.route.homeAltitude);
        if (geodetic) {
            this.item.position = geodetic;
            this.route.invoke(MapMissionsEvent.Changed, this.item, this.inRouteIndex());
        }
    }

    updateFromRouteItem(item: MissionRouteItem) {
        this.item = item;

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

        this.updatePosition(this.route.homeAltitude)

        this.billboard.setIcon(icon);
        this.circle.setRadius(loiterRadius);
        this.circle.setVisible(loiterRadius > 0);
    }

    updatePosition(homeAltitude: number) {
        if (!this.item) {
            return;
        }
        const cartesian = this.item.position ?
            cartesianFromGeodetic(this.item.position, this.route.homeAltitude) :
            Cesium.Cartesian3.ZERO;

        this.pylon.setCartesian(cartesian);
        this.billboard.setCartesian(cartesian);
        this.circle.setCartesian(cartesian);
    }

    updateProgress(reached: boolean, current: boolean) {
        const actual = true; // TODO: implement
        this.billboard.setBaseColor(current ? Cesium.Color.MAGENTA : actual ? reached ?
            Cesium.Color.AQUAMARINE : Cesium.Color.WHITE : Cesium.Color.YELLOW);
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
    constructor(missionId: string, root: MapMissionsCesium) {
        this.missionId = missionId;
        this.root = root;
        this.homeAltitude = 0;

        this.items = [];
        this.tracks = [];
    }

    done() {
        this.tracks.forEach(track => this.root.cesium.entities.remove(track))
        this.items.forEach(item => item.done());
        this.items = [];
    }

    invoke(event: MapMissionsEvent, item: MissionRouteItem, index: number) {
        this.root.invoke(event, this.missionId, item, index);
    }

    fitOnMap() {
        // TODO: implement
    }

    updateFromRoute(route: MissionRoute) {
        // Remove extra items
        while (this.items.length > route.items.length) {
            this.deleteRouteItem(this.items.length - 1);
        }
        // Add missing items
        while (this.items.length < route.items.length) {
            this.addRouteItem()
        }

        route.items.forEach((item, i) => {
            this.items[i].updateFromRouteItem(item);
        });
    }

    updateFromProgress(progress: MissionProgress) {
        this.items.forEach((item, i) => {
            const reached = progress.reached.includes(i);
            const current = progress.current === i;
            item.updateProgress(reached, current);
        });
    }

    setHomeAltitude(altitude: number) {
        if (this.homeAltitude === altitude) {
            return;
        }
        this.homeAltitude = altitude;
        this.items.forEach(item => item.updatePosition(altitude));
    }

    addRouteItem() {
        this.items.push(new MapMissionRouteItemCesium(this, this.root.cesium, this.root.interaction));
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
        const line = this.root.cesium.entities.add({
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
        this.root.cesium.entities.remove(this.tracks[index]);
        this.tracks.splice(index, 1);
    }

    indexOfRouteItem(item: MapMissionRouteItemCesium): number {
        return this.items.indexOf(item);
    }

    missionId: string
    root: MapMissionsCesium

    homeAltitude: number

    private items: Array<MapMissionRouteItemCesium>
    private tracks: Array<Cesium.Entity>
}

