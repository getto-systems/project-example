import { ConvertLocationResult } from "../../../../z_details/_ui/location/data"
import { ApplicationTargetPath, ParseVersionResult, Version } from "./data"

export function detectApplicationTargetPath(
    currentURL: URL,
    version: string,
): ConvertLocationResult<ApplicationTargetPath> {
    const prefix = `/${version}/`
    const pathname = currentURL.pathname
    if (!pathname.startsWith(prefix)) {
        // TODO これの接続テストをしていない
        const target = currentURL.searchParams.get("-application-target")
        if (target === null) {
            return { valid: false }
        } else {
            return { valid: true, value: mark(target) }
        }
    }

    return {
        valid: true,
        value: mark([pathname.replace(prefix, "/"), currentURL.search, currentURL.hash].join("")),
    }

    function mark(target: string): ApplicationTargetPath {
        return target as ApplicationTargetPath
    }
}

export function versionConfigConverter(version: string): ParseVersionResult {
    if (!version.match(/^[0-9]+\.[0-9]+\.[0-9]+([-+].*)?/)) {
        return { valid: false }
    }

    const splits = version.split(".")

    return {
        valid: true,
        value: markVersion({
            major: parseInt(splits[0]),
            minor: parseInt(splits[1]),
            patch: parseInt(splits[2]),
            suffix: suffix(splits[2], splits.slice(3)),
        }),
    }

    function suffix(patch: string, additional: string[]) {
        const suffix = patch.replace(/^[0-9]+/, "")
        return [suffix, ...additional].join(".")
    }
}

type Version_data = Readonly<{ major: number; minor: number; patch: number; suffix: string }>
function markVersion(version: Version_data): Version {
    return version as Version
}
