import { writable, get } from 'svelte/store';

import type { WsListener } from "$datasource/ws";
import { EventsService } from "$services/events";
import { CommandService } from '$services/commands';
import type { VehicleCommand, VehicleCommandState } from '$bindings/commands';

export const commands = function () {
    let commandUpdated: WsListener;
    let commandRemoved: WsListener;

    const store = writable(new Map<string, VehicleCommandState>(), (_, update) => {
        commandUpdated = (data: any) => {
            let command = data["command"] as VehicleCommandState;
            if (!command) {
                return;
            }

            update(commands => {
                commands.set(command.id, command);
                return commands;
            });
        }

        commandRemoved = (data: any) => {
            let command_id = data["command_id"] as string;
            if (!command_id) {
                return;
            }

            update(commands => {
                if (commands.has(command_id)) {
                    commands.delete(command_id);
                }
                return commands;
            });
        }

        EventsService.subscribe("LinkUpdated", commandUpdated);
        EventsService.subscribe("LinkRemoved", commandRemoved);
    });

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        state: (commandId: string) => get(store).get(commandId),
        states: () => get(store).values(),
        executeCommand: async (vehicleId: string, command: VehicleCommand) => {
            let state = await CommandService.executeCommand(vehicleId, command);
            if (state) {
                store.update(commands => commands.set(state!.id, state!));
            }
            return state;
        },
        stopCommand: async (commandId: string) => {
            let state = await CommandService.stopCommand(commandId);
            if (state) {
                store.update(commands => commands.set(state!.id, state!));
            }
            return state;
        },
        kill: () => {
            EventsService.unsubscribe("LinkUpdated", commandUpdated);
            EventsService.unsubscribe("LinkRemoved", commandRemoved);
        }
    }
} ()
