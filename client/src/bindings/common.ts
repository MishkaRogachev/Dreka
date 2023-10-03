
export type MultiId = {
    Empty: null
    String: string
};

export interface Idd {
    id: MultiId
    tb: string
}

export function iddToString(idd: Idd) {
    return idd.tb + ":" + idd.id.String;
}
