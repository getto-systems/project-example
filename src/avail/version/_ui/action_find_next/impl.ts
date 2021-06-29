import { FindNextVersionView, FindNextVersionResource } from "./resource"

export function initFindNextVersionView(
    resource: FindNextVersionResource,
): FindNextVersionView {
    return {
        resource,
        terminate: () => {
            resource.findNext.terminate()
        },
    }
}
