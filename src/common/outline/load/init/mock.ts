import { detectMenuTargetPath } from "../convert"

import { LoadBreadcrumbListShell, LoadMenuShell } from "../action"

export function mockLoadBreadcrumbListShell(
    currentURL: URL,
    version: string,
): LoadBreadcrumbListShell {
    return {
        detectTargetPath: () => detectMenuTargetPath(currentURL, version),
    }
}

export function mockLoadMenuShell(url: URL, version: string): LoadMenuShell {
    return {
        detectTargetPath: () => detectMenuTargetPath(url, version),
    }
}
