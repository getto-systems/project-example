import { detectApplicationTargetPath } from "../convert"

import { FindNextVersionShell } from "../action"

export function mockFindNextVersionShell(currentURL: URL, version: string): FindNextVersionShell {
    return {
        detectTargetPath: () => detectApplicationTargetPath(currentURL, version),
    }
}
