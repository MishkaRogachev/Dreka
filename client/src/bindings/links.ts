
export type LinkType =
    { "Udp": { address: string, port: number, } } |
    { "Tcp": { address: string, port: number, } } |
    { "Serial": { port: string, baud_rate: number, } };

export type ProtocolVersion = "MavlinkV1" | "MavlinkV2";

export type LinkProtocol = {
    "Mavlink": {
        link_type: LinkType,
        protocol_version: ProtocolVersion,
    }
};

export interface LinkDescription {
    id: [string, string] | null,
    protocol: LinkProtocol,
    enabled: boolean
};
