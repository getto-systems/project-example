import { DocsView, DocsResource } from "./resource"

export function initDocsView(resource: DocsResource): DocsView {
    return {
        resource,
        terminate: () => {
            resource.menu.terminate()
        },
    }
}
