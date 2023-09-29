import { readable, writable, get, type Writable } from 'svelte/store';

import { type VehicleDescription, VehicleType } from '$bindings/vehicles';
import { VehiclesService } from '$services/vehicles';

export const availableVehicles = readable(Array<VehicleDescription>(), (set) => {
    const interval = setInterval(() => {
        VehiclesService.getVehicles().then((vehicles: Array<VehicleDescription>) => { set(vehicles); });
    }, 1000);

    return () => clearInterval(interval);
})

availableVehicles.subscribe((vehicles: Array<VehicleDescription>) => {
    if (vehicles.length > 0 && get(selectedVehicle) === null) {
        selectedVehicle.set(vehicles[0]);
    }
})

export const selectedVehicle: Writable<VehicleDescription | null> = writable(null)

export async function addNewVehicle() {
    let vehicle = await VehiclesService.addVehicle({
        name: "Vehicle " + (get(availableVehicles).length + 1),
        protocol_id: "",
        online: false,
        vehicle_type: VehicleType.FixedWing,
        features: []
    });
    selectedVehicle.set(vehicle);
}
