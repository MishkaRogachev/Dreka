import * as Cesium from "cesium";

import { MapInteractionCesium } from "$lib/map/cesium/interaction";
import { PylonEntity, BillboardEntity } from "$lib/map/cesium/base-entities"

export class MapSign {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium, icon: string,
        cb?: (cartesian: Cesium.Cartesian3) => void) {
        this.interaction = interaction;

        this.pylon = new PylonEntity(cesium, 4.0);
        this.pylon.setBaseColor(Cesium.Color.LIGHTSTEELBLUE);

        this.sign = new BillboardEntity(cesium);
        interaction.addInteractable(this.sign);
        this.sign.setIcon(icon);
        this.sign.setDraggable(true);
        this.sign.subscribeDragging((cartesian: Cesium.Cartesian3) => {
            this.pylon.setCartesian(cartesian);
        })
        this.sign.subscribeDragged((cartesian: Cesium.Cartesian3) => {
            this.pylon.setCartesian(cartesian);
            if (cb) cb(cartesian);
        });
    }

    done() {
        this.interaction.removeInteractable(this.sign)
        this.sign.done()
        this.pylon.done()
    }

    setEnabled(enabled: boolean) {
        this.sign.setDraggable(enabled)
    }

    setCartesian(targetCartesian: Cesium.Cartesian3) {
        if (this.sign.isDragging())
            return;

        this.sign.setCartesian(targetCartesian)
        this.pylon.setCartesian(targetCartesian)
    }

    setSignColor(color: Cesium.Color) {
        this.sign.setBaseColor(color)
    }

    private sign: BillboardEntity
    private pylon: PylonEntity

    private interaction: MapInteractionCesium
}
