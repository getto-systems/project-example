import { ApplicationAction, ApplicationView } from "./action"

export function toApplicationView<R extends ApplicationAction>(resource: R): ApplicationView<R> {
    return {
        resource,
        terminate: () => {
            resource.terminate()
        },
    }
}
