import { ConvertLocationResult } from "./data"

export type LocationOutsideFeature = Readonly<{
    currentLocation: Location
}>

export type LocationTypes<T> = {
    detecter: LocationDetecter<T>
    method: LocationDetectMethod<T>
    info: T
}

export interface LocationDetecter<T> {
    (): ConvertLocationResult<T>
}
export interface LocationDetectMethod<T> {
    (currentURL: URL): ConvertLocationResult<T>
}

export interface LocationConverter<T, V> {
    (value: V): ConvertLocationResult<T>
}
