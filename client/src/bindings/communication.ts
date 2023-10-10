export type LinkType =
    { "Udp": { address: string, port: number, } } |
    { "Tcp": { address: string, port: number, } } |
    { "Serial": { port: string, baud_rate: number, } };

export enum MavlinkProtocolVersion {
    MavlinkV1 = "MavlinkV1",
    MavlinkV2 = "MavlinkV2",
}

export type LinkProtocol = {
    Mavlink: {
        link_type: LinkType,
        protocol_version: MavlinkProtocolVersion,
    }
};

export interface LinkDescription {
    id: string,
    protocol: LinkProtocol,
    enabled: boolean,
    name: String
};

export interface LinkStatus {
    id: string,
    is_connected: boolean,
    is_online: boolean
}
