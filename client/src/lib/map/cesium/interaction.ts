import { type MapInteraction, type ClickListener } from '$lib/interfaces/map';
import { GeodeticFrame, type Cartesian } from "$bindings/spatial";

import * as Cesium from 'cesium';

const _pickingRadius: number = 25;

export enum KeyModifier {
    None,
    Ctrl,
    Shift,
    Option
}

export interface Interactable {
    setDragging: (dragging: boolean) => void
    setHovered: (hovered: boolean) => void

    matchInteraction: (objects: Array<any>) => boolean

    drag: (screenXY: Cesium.Cartesian2, modifier: KeyModifier) => boolean
    click: () => boolean

    isDraggable: () => boolean
    isDragging: () => boolean
    isHoverable: () => boolean
}

export class MapInteractionCesium implements MapInteraction {
    constructor(cesium: Cesium.Viewer) {
        this.cesium = cesium;

        this.mouseXY = { x: NaN, y: NaN };
        this.interactables = [];
        this.clickListeners = [];
        this.draggingInteractable = null;
        this.hoveredInteractable = null;

        // Remove conflicting default behavior
        this.cesium.screenSpaceEventHandler.removeInputAction(Cesium.ScreenSpaceEventType.LEFT_DOUBLE_CLICK);
        this.cesium.screenSpaceEventHandler.removeInputAction(Cesium.ScreenSpaceEventType.LEFT_CLICK,
                                                                Cesium.KeyboardEventModifier.CTRL);
        this.cesium.screenSpaceEventHandler.removeInputAction(Cesium.ScreenSpaceEventType.MOUSE_MOVE,
                                                                Cesium.KeyboardEventModifier.CTRL);
        this.cesium.scene.screenSpaceCameraController.enableLook = false;

        // Subscribe with all modifiers
        this.subscribeInputActions((event: any, modifier: KeyModifier) => {
            if (Cesium.defined(event.endPosition)) {
                this.mouseXY = { x: event.endPosition.x, y: event.endPosition.y };
                this.onMove(event.endPosition, modifier);
            } else {
                this.mouseXY = { x: NaN, y: NaN };
            }
        }, Cesium.ScreenSpaceEventType.MOUSE_MOVE);

        this.subscribeInputActions((event: any, modifier: KeyModifier) => {
            if (Cesium.defined(event.position)) {
                this.mouseXY = { x: event.position.x, y: event.position.y };
                this.onUp(event.position);
            }
        }, Cesium.ScreenSpaceEventType.LEFT_UP);

        this.subscribeInputActions((event: any, modifier: KeyModifier) => {
            if (Cesium.defined(event.position)) {
                this.mouseXY = { x: event.position.x, y: event.position.y };
                this.onDown(event.position);
            }
        }, Cesium.ScreenSpaceEventType.LEFT_DOWN);

        this.subscribeInputActions((event: any, modifier: KeyModifier) => {
            if (Cesium.defined(event.position)) {
                this.mouseXY = { x: event.position.x, y: event.position.y };
                this.onClick(event.position);
            }
        }, Cesium.ScreenSpaceEventType.LEFT_CLICK);
    }

    mouseCoordinates(): Cartesian {
        return this.mouseXY;
    }

    addInteractable(interactable: Interactable) {
        this.interactables.push(interactable);
    }

    removeInteractable(interactable: Interactable) {
        const index = this.interactables.indexOf(interactable);
        if (index > -1)
            this.interactables.splice(index, 1);
    }

    hoverInteractable(interactable: Interactable | null) {
        if (this.hoveredInteractable === interactable)
            return;

        // Drop old hover
        if (this.hoveredInteractable)
            this.hoveredInteractable.setHovered(false);

        // Hover newone
        if (interactable && (interactable.isHoverable() || interactable.isDraggable())) {
            interactable.setHovered(true);
            this.hoveredInteractable = interactable;
            return;
        }

        // Or nothing
        this.hoveredInteractable = null;
    }

    subscribeClick(listener: ClickListener) {
        this.clickListeners.push(listener);
    }

    unsubscribeClick(listener: ClickListener) {
        this.clickListeners = this.clickListeners.filter(item => item !== listener);
    }

    onMove(position: Cesium.Cartesian2, modifier: KeyModifier): boolean {
        // If dragging, just do it
        if (this.draggingInteractable) {
            if (!this.draggingInteractable.isDragging()) {
                this.draggingInteractable.setDragging(true);
            }
            return this.draggingInteractable.drag(position, modifier);
        }

        this.hoverInteractable(this.firstMatchPickingInteractable(position));
        return false;
    }

    onUp(position: Cesium.Cartesian2): boolean {
        if (this.draggingInteractable) {
            this.setDragging(null);
            this.hoverInteractable(this.firstMatchPickingInteractable(position));
            return true;
        }
        return false;
    }

    onDown(position: Cesium.Cartesian2): boolean {
        const interactable = this.firstMatchPickingInteractable(position);
        if (interactable && interactable.isDraggable()) {
            this.setDragging(interactable);
            return true;
        }
        return false;
    }

    onClick(position: Cesium.Cartesian2) {
        if (this.hoveredInteractable) {
            this.hoveredInteractable.click();
            return;
        }

        const cartesian = this.cartesianFromPosition(position);
        if (!cartesian)
            return;

        const cartographic = Cesium.Cartographic.fromCartesian(cartesian);
        const geodetic = {
            latitude: Cesium.Math.toDegrees(cartographic.latitude),
            longitude: Cesium.Math.toDegrees(cartographic.longitude),
            altitude: cartographic.height,
            frame: GeodeticFrame.Wgs84AboveSeaLevel
        }
        for (const listener of this.clickListeners) {
            if (listener(geodetic, position))
                return;
        }
    }

    firstMatchPickingInteractable(position: Cesium.Cartesian2) {
        const objects = this.cesium.scene.drillPick(position, undefined, _pickingRadius, _pickingRadius);
        if (!objects.length)
            return null;

        for (const interactable of this.interactables) {
            if (interactable.matchInteraction(objects)) {
                return interactable;
            }
        }
        return null;
    }

    setDragging(interactable: Interactable | null) {
        if (this.draggingInteractable)
            this.draggingInteractable.setDragging(false);

        this.draggingInteractable = interactable;
        this.cesium.scene.screenSpaceCameraController.enableInputs = interactable === null;
    }

    cartesianFromPosition(position: Cesium.Cartesian2): Cesium.Cartesian3 | undefined {
        const ray = this.cesium.camera.getPickRay(position);
        if (!ray)
            return undefined;

        return this.cesium.scene.globe.pick(ray, this.cesium.scene);
    }

    subscribeInputActions(cb: (event: any, modifier: KeyModifier) => void, type: Cesium.ScreenSpaceEventType) {
        this.cesium.screenSpaceEventHandler.setInputAction((event: any) => { cb(event, KeyModifier.None) }, type, undefined);
        this.cesium.screenSpaceEventHandler.setInputAction((event: any) => { cb(event, KeyModifier.Ctrl) }, type, Cesium.KeyboardEventModifier.CTRL);
        this.cesium.screenSpaceEventHandler.setInputAction((event: any) => { cb(event, KeyModifier.Shift) }, type, Cesium.KeyboardEventModifier.SHIFT);
        this.cesium.screenSpaceEventHandler.setInputAction((event: any) => { cb(event, KeyModifier.Option) }, type, Cesium.KeyboardEventModifier.ALT);
    }

    private cesium: Cesium.Viewer

    private mouseXY: Cartesian
    private interactables: Array<Interactable>
    private clickListeners: Array<ClickListener>
    private draggingInteractable: Interactable | null
    private hoveredInteractable: Interactable | null
}
