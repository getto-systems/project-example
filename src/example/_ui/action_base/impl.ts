import { BaseView, BaseResource } from "./resource"

export function initBaseView<R>(resource: R & BaseResource, terminate: { (): void }): BaseView<R> {
    return {
        resource,
        terminate: () => {
            resource.menu.terminate()
            terminate()
        },
    }
}
