import type { MapGraticule } from '$lib/interfaces/map'

import type * as Cesium from 'cesium'
// @ts-ignore
import Graticule from 'cesium-graticule'

export class MapGraticuleCesium implements MapGraticule {
    constructor(cesium: Cesium.Viewer) {
        this.graticule = new Graticule(cesium);
        this.graticule.visible = false;
    }

    enabled(): boolean {
        return this.graticule.visible;
    }

    setEnabled(enabled: boolean) {
        this.graticule.visible = enabled;
    }

    private graticule: Graticule
}
