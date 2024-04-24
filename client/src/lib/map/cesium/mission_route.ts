import { type MissionRoute, type MissionRouteItem, MissionRouteItemType, type MissionProgress } from '$bindings/mission';

import { type MapMissionsEvent, type MapMissionRoute } from '$lib/interfaces/map';
import { MapInteractionCesium } from '$lib/map/cesium/interaction';
import { MapMissionsCesium } from '$lib/map/cesium/missions';
import { CircleEntity, type EntityInputEvent } from "$lib/map/cesium/base-entities"
import { cartesianFromGeodetic, geodeticFromCartesian } from '$lib/map/cesium/utils';
import { MapSign } from '$lib/map/cesium/common';

import * as Cesium from 'cesium';

import wptIcon from "$assets/svg/map_wpt.svg";
import takeoffIcon from "$assets/svg/map_takeoff.svg";
import landingIcon from "$assets/svg/map_landing.svg";

class MapMissionRouteItemCesium extends MapSign {
    constructor(route: MapMissionRouteCesium, cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        super(cesium, interaction);
        this.route = route;

        this.setDragCallback((cartesian: Cesium.Cartesian3) => {
            if (!this.item?.position) {
                return;
            }
            const geodetic = geodeticFromCartesian(cartesian, this.item!.position!.frame, this.route.homeAltitude);
            if (geodetic) {
                this.item.position = geodetic;
                this.route.invoke({ ChangesOrdered: { missionId: route.missionId, item: this.item, index: this.inRouteIndex() } });
            }
        });

        this.billboard.subscribe((event: EntityInputEvent) => {
            if (event.Clicked && this.item) {
                this.route.invoke({ InvokeWaypointMenu: { missionId: route.missionId, item: this.item, index: this.inRouteIndex() } });
            }
            if (event.Hovered && this.item) {
                this.route.invoke({ Hovered: { missionId: route.missionId, item: this.item, index: this.inRouteIndex() } });
            } else if (event.Exited) {
                this.route.invoke({ Exited: { missionId: route.missionId, index: this.inRouteIndex() } });
            }
            if (event.DraggedPosition) {
                const geodetic = geodeticFromCartesian(event.DraggedPosition.cartesian, this.item!.position!.frame, this.route.homeAltitude);
                if (geodetic && this.item) {
                    this.route.invoke({ WaypointDragged: {
                        missionId: route.missionId,
                        item: this.item,
                        index: this.inRouteIndex(),
                        position: geodetic
                    } });
                }
            }
        });

        this.circle = new CircleEntity(cesium, 4.0);
        // TODO: add circle interaction
        // this.circle.setDraggable(true)
        // this.circle.subscribeDragged((radius: number) => { this.onRadiusUpdated(radius) })
        // this.interaction.addInteractable(this.circle)
    }

    done() {
        //this.interaction.removeInteractable(this.circle)
        this.circle.done();
        super.done();
    }

    setCartesian(cartesian: Cesium.Cartesian3): boolean {
        super.setCartesian(cartesian)
        this.circle.setCartesian(cartesian);
        return true;
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

        this.setIcon(icon);
        this.circle.radius = loiterRadius;
        this.circle.visible = (loiterRadius > 0);
    }

    updatePosition(homeAltitude: number) {
        if (!this.item) {
            return;
        }
        const cartesian = this.item.position ?
            cartesianFromGeodetic(this.item.position, this.route.homeAltitude) :
            Cesium.Cartesian3.ZERO;

        this.setCartesian(cartesian);
    }

    updateProgress(reached: boolean, current: boolean) {
        const actual = true; // TODO: implement
        this.billboard.baseColor = current ? Cesium.Color.MAGENTA : actual ? reached ?
            Cesium.Color.LIGHTCYAN : Cesium.Color.WHITE : Cesium.Color.YELLOW;
    }

    inRouteIndex(): number {
        return this.route.indexOfRouteItem(this);
    }

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

    invoke(event: MapMissionsEvent) {
        this.root.invoke(event);
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

    updateFromProgress(progress: MissionProgress, inMissionMode: boolean) {
        this.items.forEach((item, i) => {
            const reached = progress.reached.includes(i);
            const current = inMissionMode && progress.current === i;
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
                    let list = []
                    if (first.ordered.isVisibleAndPositionValid()) {
                        list.push(first.ordered.cartesian())
                    } else if (first.isVisibleAndPositionValid()) {
                        list.push(first.cartesian())
                    }

                    if (second.ordered.isVisibleAndPositionValid()) {
                        list.push(second.ordered.cartesian())
                    } else if (second.isVisibleAndPositionValid()) {
                        list.push(second.cartesian())
                    }
                    return list
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

    homeAltitude: number // TODO: move to Misson

    private items: Array<MapMissionRouteItemCesium>
    private tracks: Array<Cesium.Entity>
}
