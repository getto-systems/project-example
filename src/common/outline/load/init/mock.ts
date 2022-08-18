import { detectMenuTargetPath } from "../convert"

import { OutlineBreadcrumbListShell, OutlineMenuShell } from "../action"

export function mockOutlineBreadcrumbListShell(
    currentURL: URL,
    version: string,
): OutlineBreadcrumbListShell {
    return {
        detectTargetPath: () => detectMenuTargetPath(currentURL, version),
    }
}

export function mockOutlineMenuShell(url: URL, version: string): OutlineMenuShell {
    return {
        detectTargetPath: () => detectMenuTargetPath(url, version),
    }
}
