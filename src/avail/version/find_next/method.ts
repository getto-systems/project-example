import { delayedChecker } from "../../../z_details/_ui/timer/helper"

import { versionToString } from "./helper"

import { CheckDeployExistsRemote, FindNextVersionInfra } from "./infra"

import { FindNextVersionEvent } from "./event"

import { versionStringConfigConverter } from "../convert"
import { versionConfigConverter } from "./convert"

import { ConvertLocationResult } from "../../../z_details/_ui/location/data"
import { ApplicationTargetPath, CheckDeployExistsRemoteError, Version } from "./data"

export interface FindNextVersionPod {
    (detecter: FindNextVersionDetecter): FindNextVersionMethod
}
export interface FindNextVersionMethod {
    <S>(post: Post<FindNextVersionEvent, S>): Promise<S>
}

export type FindNextVersionDetecter = Detect<ApplicationTargetPath>

interface Find {
    (infra: FindNextVersionInfra): FindNextVersionPod
}
export const findNextVersion: Find = (infra) => (detecter) => async (post) => {
    const { version, versionSuffix, config } = infra

    const target = detecter()
    const currentVersion = versionConfigConverter(version)

    if (!currentVersion.valid) {
        return post({
            type: "succeed-to-find",
            upToDate: true,
            version: versionStringConfigConverter(version),
            target,
        })
    }

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const next = await delayedChecker(
        findNext(infra.check, currentVersion.value, versionSuffix),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime-to-find" }),
    )
    if (!next.success) {
        return post({ type: "failed-to-find", err: next.err })
    }

    if (!next.found) {
        return post({
            type: "succeed-to-find",
            upToDate: true,
            version: versionStringConfigConverter(version),
            target,
        })
    } else {
        return post({
            type: "succeed-to-find",
            upToDate: false,
            version: versionToString(next.version),
            target,
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
        // 自動で major バージョンアップをするとまずいので minor バージョンのチェックから行う
        const response = await checkVersion(nextMinorVersion(current, suffix))
        if (!response.success) {
            return response
        }
        if (response.found) {
            return response
        }
        // minor バージョンが見つからなかったら patch バージョンのチェックを行う
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
interface Detect<T> {
    (): ConvertLocationResult<T>
}
