import { NotFoundView, NotFoundResource } from "./resource"

export function initNotFoundView(resource: NotFoundResource): NotFoundView {
    return {
        resource,
        terminate: () => {
            // 特に terminate するものはない
        }
    }
}
