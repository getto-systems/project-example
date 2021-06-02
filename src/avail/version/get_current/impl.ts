import { versionStringConfigConverter } from "../converter"

import { GetCurrentVersionInfra } from "./infra"

import { GetCurrentVersionMethod } from "./method"

interface GetCurrentVersion {
    (infra: GetCurrentVersionInfra): GetCurrentVersionMethod
}
export const getCurrentVersion: GetCurrentVersion = (infra) => () => {
    const { version } = infra
    return versionStringConfigConverter(version)
}
