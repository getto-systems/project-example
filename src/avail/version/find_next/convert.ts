import { ConvertLocationResult } from "../../../z_lib/ui/location/data"
import { ApplicationTargetPath, ParseVersionResult, Version } from "./data"

export function detectApplicationTargetPath(
    currentURL: URL,
    version: string,
): ConvertLocationResult<ApplicationTargetPath> {
    const target = currentURL.searchParams.get("-application-target")
    if (target !== null) {
        return { valid: true, value: specify(target) }
    }

    const prefix = `/${version}/`
    const pathname = currentURL.pathname

    if (!pathname.startsWith(prefix)) {
        return { valid: false }
    }
    const path = pathname.replace(prefix, "/")

    return {
        valid: true,
        value: detect(path),
    }

    function specify(path: string): ApplicationTargetPath {
        return { path, specified: true } as ApplicationTargetPath
    }
    function detect(path: string): ApplicationTargetPath {
        return {
            path: [path, currentURL.search, currentURL.hash].join(""),
            specified: false,
        } as ApplicationTargetPath
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

    function suffix(patch: string, additional: readonly string[]) {
        const suffix = patch.replace(/^[0-9]+/, "")
        return [suffix, ...additional].join(".")
    }
}

type Version_data = Readonly<{ major: number; minor: number; patch: number; suffix: string }>
function markVersion(version: Version_data): Version {
    return version as Version
}
