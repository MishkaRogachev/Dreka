import { writable, get } from 'svelte/store';

import type { Command, CommandExecution, CommandExecutor } from '$bindings/commands';

import type { WsListener } from "$datasource/ws";
import { ClientSideEvents, EventsService } from "$services/events";
import { CommandService } from '$services/commands';

export const commandExecutions = function () {
    let executionUpdated: WsListener;
    let executionRemoved: WsListener;
    let wsConnected: WsListener;

    const store = writable(new Map<string, CommandExecution>(), (_, update) => {
        executionUpdated = (data: any) => {
            let execution = data["execution"] as CommandExecution;
            if (!execution) {
                return;
            }

            update(executions => {
                executions.set(execution.id, execution);
                return executions;
            });
        }

        executionRemoved = (data: any) => {
            let id = data["command_id"] as string;
            if (!id) {
                return;
            }

            update(executions => {
                if (executions.has(id)) {
                    executions.delete(id);
                }
                return executions;
            });
        }

        wsConnected = async (_data: any) => {
            let executions = await CommandService.getCommandExecutions();
            if (executions) {
                let executionsMap = new Map(executions!.map(execution => [execution.id, execution]));
                update(_ => { return executionsMap; });
            }
        }

        EventsService.subscribe("CommandExecutionUpdated", executionUpdated);
        EventsService.subscribe("CommandExecutionRemoved", executionRemoved);
        EventsService.subscribe(ClientSideEvents.WsConnectionOpened, wsConnected);
    });

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        execution: (id: string) => get(store).get(id),
        executions: () => get(store).values(),
        executeCommand: async (command: Command, executor: CommandExecutor) => {
            return await CommandService.executeCommand({ command, executor });
        },
        cancelCommand: async (id: string) => {
            return await CommandService.cancelCommand(id);
        },
        kill: () => {
            EventsService.unsubscribe("CommandExecutionUpdated", executionUpdated);
            EventsService.unsubscribe("CommandExecutionRemoved", executionRemoved);
            EventsService.unsubscribe(ClientSideEvents.WsConnectionOpened, wsConnected);
        }
    }
} ()
