export interface SocketData { address: string, port: number }
export interface SerialData { port: string, baud_rate: number }

export type LinkType = {
    Udp?: SocketData,
    Tcp?: SocketData,
    Serial?: SerialData
};

export enum MavlinkProtocolVersion {
    MavlinkV1 = "MavlinkV1",
    MavlinkV2 = "MavlinkV2",
}

export interface MavlinkProtocol {
    link_type: LinkType,
    protocol_version: MavlinkProtocolVersion
}

export type LinkProtocol = {
    Mavlink?: MavlinkProtocol
};

export interface LinkDescription {
    id?: string,
    protocol: LinkProtocol,
    enabled: boolean,
    name: String
};

export interface LinkStatus {
    id: string,
    is_connected: boolean,
    is_online: boolean
}
