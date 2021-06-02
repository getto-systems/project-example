import { newGetCurrentVersionResource } from "../../version/action_get_current/init"

import { initNotFoundView } from "./impl"

import { NotFoundView } from "./resource"

export function newNotFoundView(): NotFoundView {
    return initNotFoundView({
        ...newGetCurrentVersionResource(),
    })
}
