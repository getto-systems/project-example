import { ApplicationView } from "../../../../ui/vendor/getto-application/action/action"

import { GetCurrentVersionResource } from "../../version/action_get_current/resource"

export type NotFoundView = ApplicationView<NotFoundResource>

export type NotFoundResource = GetCurrentVersionResource
