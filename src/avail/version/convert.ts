import { VersionString } from "./data"

export function versionStringConverter(version: string): VersionString {
    return markVersionString(version)
}

function markVersionString(version: string): VersionString {
    return version as VersionString
}
