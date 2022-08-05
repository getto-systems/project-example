import {
    ApplicationState,
    initApplicationState,
} from "../../../z_vendor/getto-application/action/action"

import { versionStringConverter } from "../kernel/convert"
import { versionConfigConverter } from "./convert"
import { versionToString } from "./helper"
import { checkTakeLongtime } from "../../../z_lib/ui/timer/helper"

import { WaitTime } from "../../../z_lib/ui/config/infra"
import { ApplicationTargetPathDetecter, CheckDeployExistsRemote } from "./infra"

import { ConvertLocationResult } from "../../../z_lib/ui/location/data"
import { VersionString } from "../kernel/data"
import {
    ApplicationTargetPath,
    CheckDeployExistsError,
    CheckDeployExistsRemoteError,
    Version,
} from "./data"

export interface FindNextVersionAction {
    readonly state: ApplicationState<FindNextVersionState>
}

export type FindNextVersionState = Readonly<{ type: "initial" }> | FindNextVersionEvent

const initialState: FindNextVersionState = { type: "initial" }

export type FindNextVersionMaterial = Readonly<{
    infra: FindNextVersionInfra
    shell: FindNextVersionShell
    config: FindNextVersionConfig
}>
export type FindNextVersionInfra = Readonly<{
    check: CheckDeployExistsRemote
}>
export type FindNextVersionShell = Readonly<{
    detectTargetPath: ApplicationTargetPathDetecter
}>
export type FindNextVersionConfig = Readonly<{
    version: string
    versionSuffix: string
    takeLongtimeThreshold: WaitTime
}>

export function initFindNextVersionAction(
    material: FindNextVersionMaterial,
): FindNextVersionAction {
    const { state, post } = initApplicationState({
        initialState,
        ignite: (): Promise<FindNextVersionState> => findNextVersion(material, post),
    })
    return { state }
}

export type FindNextVersionEvent =
    | Readonly<{ type: "take-longtime" }>
    | Readonly<{ type: "failed"; err: CheckDeployExistsError }>
    | Readonly<{
          type: "success"
          upToDate: boolean
          version: VersionString
          target: ConvertLocationResult<ApplicationTargetPath>
      }>

async function findNextVersion<S>(
    { infra, shell, config }: FindNextVersionMaterial,
    post: Post<FindNextVersionEvent, S>,
): Promise<S> {
    const { check } = infra

    const currentVersion = versionConfigConverter(config.version)

    if (!currentVersion.valid) {
        return post({
            type: "success",
            upToDate: true,
            version: versionStringConverter(config.version),
            target: shell.detectTargetPath(),
        })
    }

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const next = await checkTakeLongtime(
        findNext(check, currentVersion.value, config.versionSuffix),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime" }),
    )
    if (!next.success) {
        return post({ type: "failed", err: next.err })
    }

    if (!next.found) {
        return post({
            type: "success",
            upToDate: true,
            version: versionStringConverter(config.version),
            target: shell.detectTargetPath(),
        })
    } else {
        return post({
            type: "success",
            upToDate: false,
            version: versionToString(next.version),
            target: shell.detectTargetPath(),
        })
    }
}

type FindNextResult =
    | Readonly<{ success: true; found: true; version: Version }>
    | Readonly<{ success: true; found: false }>
    | Readonly<{ success: false; err: CheckDeployExistsRemoteError }>

async function findNext(
    check: CheckDeployExistsRemote,
    current: Version,
    suffix: string,
): Promise<FindNextResult> {
    let result = await checkNext(current, suffix)

    while (result.success && result.found) {
        const next = await checkNext(result.version, suffix)
        if (!next.success || !next.found) {
            break
        }
        result = next
    }

    return result

    async function checkNext(current: Version, suffix: string): Promise<FindNextResult> {
        // まず次の major バージョンがあるか確認
        const majorResponse = await checkVersion(nextMajorVersion(current, suffix))
        if (!majorResponse.success) {
            return majorResponse
        }
        if (majorResponse.found) {
            return majorResponse
        }

        // 次に minor バージョン
        const minorResponse = await checkVersion(nextMinorVersion(current, suffix))
        if (!minorResponse.success) {
            return minorResponse
        }
        if (minorResponse.found) {
            return minorResponse
        }

        // 最後に patch バージョン
        return await checkVersion(nextPatchVersion(current, suffix))
    }
    async function checkVersion(version: Version): Promise<FindNextResult> {
        const response = await check(checkURL(version))
        if (!response.success) {
            return response
        }
        if (!response.value.found) {
            return { success: true, found: false }
        }
        return { success: true, found: true, version }
    }
}

function nextMajorVersion(version: Version, suffix: string): Version {
    return {
        ...version,
        major: version.major + 1,
        minor: 0,
        patch: 0,
        suffix,
    }
}
function nextMinorVersion(version: Version, suffix: string): Version {
    return {
        ...version,
        major: version.major,
        minor: version.minor + 1,
        patch: 0,
        suffix,
    }
}
function nextPatchVersion(version: Version, suffix: string): Version {
    return {
        ...version,
        major: version.major,
        minor: version.minor,
        patch: version.patch + 1,
        suffix,
    }
}

function checkURL(version: Version): string {
    return `/${versionToString(version)}/index.html`
}

interface Post<E, S> {
    (event: E): S
}
