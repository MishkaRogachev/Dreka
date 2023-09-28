import { readable, writable, type Writable } from 'svelte/store';

import { type VehicleDescription, VehicleType } from '$bindings/vehicles';
import { VehiclesService } from '$services/vehicles';

export const availableVehicles = readable(Array<VehicleDescription>(), (set) => {
    const interval = setInterval(() => {
        VehiclesService.getVehicles().then((vehicles: any) => {
            set(vehicles as Array<VehicleDescription>);
        });
    }, 1000);

    return () => clearInterval(interval);
})

export const selectedVehicle: Writable<VehicleDescription | null> = writable(null)

export async function addNewVehicle() {
    await VehiclesService.addVehicle({
        name: "New Vehicle",
        protocol_id: "",
        online: false,
        vehicle_type: VehicleType.Plane,
        features: []
    });
}
