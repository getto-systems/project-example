import { detectMenuTargetPath } from "../convert"

import { LoadBreadcrumbListShell, OutlineMenuShell } from "../action"

export function mockLoadBreadcrumbListShell(
    currentURL: URL,
    version: string,
): LoadBreadcrumbListShell {
    return {
        detectTargetPath: () => detectMenuTargetPath(currentURL, version),
    }
}

export function mockOutlineMenuShell(url: URL, version: string): OutlineMenuShell {
    return {
        detectTargetPath: () => detectMenuTargetPath(url, version),
    }
}
