
export function countDecimals(value: number): number {
    if (Math.floor(value) === value)
        return 0
    return value.toString().split(".")[1].length || 0
}

export function roundPrecision(value: number, precision: number): number {
    const decimals = Math.pow(10, precision)
    return Math.round((value + Number.EPSILON) * decimals) / decimals
}

export function pad(value: number, size: number): string {
    let str = value.toString()
    while (str.length < size) {
        str = "0" + str
    }
    return str
}

export function formatHeading(angle: number) {
    return angle = (angle + 360) % 360
}

export interface Dms {
    sign: -1 | 1
    deg: number
    min: number
    sec: number
}

export function degreesToDms(degrees: number, isLongitude: boolean, secondsPrecision: number): Dms {
    const sign = degrees < 0 ? -1 : 1
    const abs = Math.abs(degrees)
    let deg = Math.floor(abs)
    let frac = (abs - deg) * 60

    if (Math.ceil(frac) - frac <= Math.pow(10, -7))
        frac = Math.ceil(frac)

    const min = Math.floor(frac)
    let sec = (frac - min) * 60

    const degLimit = (isLongitude ? 180 : 90)
    if (deg > degLimit)
        deg = degLimit

    return {
        sign: sign,
        deg: deg,
        min: min,
        sec: roundPrecision(sec, secondsPrecision)
    }
}

export function degreesToDmsString(degrees: number, isLongitude: boolean): string {
    if (isNaN(degrees))
        return "-"

    const dms = degreesToDms(degrees, isLongitude, 2)
    return pad(dms.deg, isLongitude ? 3 : 2) + "\u00B0" + pad(dms.min, 2) + "\'" +
            pad(dms.sec, 3 + 2) + "\"" +
            (dms.sign < 0 ? isLongitude ? "W" : "S" : isLongitude ? "E" : "N")
}

export function roundTo125(value: number): number
{
    const magnitude = Math.floor(Math.log(value) / Math.LN10)
    let result = value / Math.pow(10.0, magnitude)

    if (result < 2.0)
        result = 1.0
    else if (result < 5.0)
        result = 2.0
    else if (result < 10.0)
        result = 5.0
    else
        result = 10.0
    result *= Math.pow(10.0, magnitude)
    return result
}