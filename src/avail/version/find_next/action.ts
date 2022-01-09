import {
    StatefulApplicationAction,
    AbstractStatefulApplicationAction,
} from "../../../../ui/vendor/getto-application/action/action"
import { DelayTime } from "../../../z_lib/ui/config/infra"
import { ConvertLocationResult } from "../../../z_lib/ui/location/data"
import { delayedChecker } from "../../../z_lib/ui/timer/helper"
import { versionStringConverter } from "../kernel/convert"
import { VersionString } from "../kernel/data"
import { versionConfigConverter } from "./convert"
import {
    ApplicationTargetPath,
    CheckDeployExistsError,
    CheckDeployExistsRemoteError,
    Version,
} from "./data"
import { versionToString } from "./helper"
import { ApplicationTargetPathDetecter, CheckDeployExistsRemote } from "./infra"

export type FindNextVersionAction = StatefulApplicationAction<FindNextVersionState>

export type FindNextVersionState = Readonly<{ type: "initial-next-version" }> | FindNextVersionEvent

export const initialFindNextVersionState: FindNextVersionState = {
    type: "initial-next-version",
}

export type FindNextVersionConfig = Readonly<{
    takeLongtimeThreshold: DelayTime
}>
export type FindNextVersionInfra = Readonly<{
    version: string
    versionSuffix: string
    check: CheckDeployExistsRemote
}>
export type FindNextVersionShell = Readonly<{
    detectTargetPath: ApplicationTargetPathDetecter
}>

export function initFindNextVersionAction(
    config: FindNextVersionConfig,
    infra: FindNextVersionInfra,
    shell: FindNextVersionShell,
): FindNextVersionAction {
    return new Action(config, infra, shell)
}

class Action
    extends AbstractStatefulApplicationAction<FindNextVersionState>
    implements FindNextVersionAction
{
    readonly initialState = initialFindNextVersionState

    constructor(
        config: FindNextVersionConfig,
        infra: FindNextVersionInfra,
        shell: FindNextVersionShell,
    ) {
        super({
            ignite: () => findNextVersion(config, infra, shell, this.post),
        })
    }
}

export type FindNextVersionEvent =
    | Readonly<{ type: "take-longtime-to-find" }>
    | Readonly<{ type: "failed-to-find"; err: CheckDeployExistsError }>
    | Readonly<{
          type: "succeed-to-find"
          upToDate: boolean
          version: VersionString
          target: ConvertLocationResult<ApplicationTargetPath>
      }>

async function findNextVersion<S>(
    config: FindNextVersionConfig,
    infra: FindNextVersionInfra,
    shell: FindNextVersionShell,
    post: Post<FindNextVersionEvent, S>,
): Promise<S> {
    const { version, versionSuffix } = infra

    const currentVersion = versionConfigConverter(version)

    if (!currentVersion.valid) {
        return post({
            type: "succeed-to-find",
            upToDate: true,
            version: versionStringConverter(version),
            target: shell.detectTargetPath(),
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
            version: versionStringConverter(version),
            target: shell.detectTargetPath(),
        })
    } else {
        return post({
            type: "succeed-to-find",
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
