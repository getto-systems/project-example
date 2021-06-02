import { getCurrentVersion } from "../../get_current/impl"

import { GetCurrentVersionInfra } from "../../get_current/infra"

import { GetCurrentVersionCoreAction } from "./action"

export function initGetCurrentVersionCoreAction(
    infra: GetCurrentVersionInfra,
): GetCurrentVersionCoreAction {
    return {
        getCurrent: getCurrentVersion(infra),
    }
}
