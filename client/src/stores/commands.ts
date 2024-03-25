import { writable, get } from 'svelte/store';

import type { VehicleCommand, VehicleCommandState } from '$bindings/commands';
import { CommandService } from '$services/commands';

const UPDATE_STATE_INTERVAL = 100;

export const commands = function () {
    let stateInterval: NodeJS.Timeout;

    const store = writable(new Map<string, VehicleCommandState>(), (_, update) => {
        stateInterval = setInterval(async () => {
            let commandStates = await CommandService.getCommands();
            if (!commandStates) {
                store.update(commands => {
                    let newCommands = new Map<string, VehicleCommandState>();
                    commandStates!.forEach(state => newCommands.set(state.command_id, state))
                    return newCommands;
                });
            }
        }, UPDATE_STATE_INTERVAL);
    });

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        state: (commandId: string) => get(store).get(commandId),
        states: () => get(store).values(),
        executeCommand: async (vehicleId: string, command: VehicleCommand) => {
            let status = await CommandService.executeCommand(vehicleId, command);
            if (status) {
                store.update(commands => commands.set(status!.command_id, status!));
            }
            return status;
        },
        stopCommand: async (commandId: string) => {
            let status = await CommandService.stopCommand(commandId);
            if (status) {
                store.update(commands => commands.set(status!.command_id, status!));
            }
            return status;
        },
        kill: () => {
            clearInterval(stateInterval);
        }
    }
} ()
