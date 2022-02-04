import { detectMenuTargetPath } from "../../kernel/convert"

import { LoadMenuShell } from "../action"

export function mockLoadMenuShell(url: URL, version: string): LoadMenuShell {
    return {
        detectTargetPath: () => detectMenuTargetPath(url, version),
    }
}
