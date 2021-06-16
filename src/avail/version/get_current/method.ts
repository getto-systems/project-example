import { GetCurrentVersionInfra } from "./infra"

import { versionStringConfigConverter } from "../convert"

import { VersionString } from "../data"

export interface GetCurrentVersionMethod {
    (): VersionString
}

interface GetCurrentVersion {
    (infra: GetCurrentVersionInfra): GetCurrentVersionMethod
}
export const getCurrentVersion: GetCurrentVersion = (infra) => () => {
    const { version } = infra
    return versionStringConfigConverter(version)
}
