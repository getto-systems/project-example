import { ApplicationView } from "../z_vendor/getto-application/action/action"
import { DocsResource } from "./resource"

export function initDocsView(resource: DocsResource): ApplicationView<DocsResource> {
    return {
        resource,
        terminate: () => {
            resource.menu.terminate()
        },
    }
}
