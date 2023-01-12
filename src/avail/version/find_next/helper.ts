import { VersionString } from "../kernel/data"
import { ApplicationTargetPath, Version } from "./data"
import { ConvertLocationResult } from "../../../common/util/location/data"

export function applicationPath(
    version: string,
    target: ConvertLocationResult<ApplicationTargetPath>,
): string {
    const path = target.valid ? target.value.path : "/index.html"
    return `/${version}${path}`
}

export function versionToString(version: Version): VersionString {
    return `${version.major}.${version.minor}.${version.patch}${version.suffix}` as VersionString
}
