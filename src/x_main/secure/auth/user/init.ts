import { ForegroundOutsideFeature } from "../../../../x_outside_feature/common"

import { newModifyAuthUserAccountAction } from "../../../../auth/user/account/modify/init/resource"
import { newChangeResetTokenDestinationAction } from "../../../../auth/user/password/reset/token_destination/change/init/resource"
import { newOverwriteLoginIdAction } from "../../../../auth/user/login_id/change/init/resource"
import { newOverwritePasswordAction } from "../../../../auth/user/password/change/init/resource"
import { newUnregisterAuthUserAccountAction } from "../../../../auth/user/account/unregister/init/resource"

import { DetailAuthUserAccountActions } from "../../../../auth/user/account/kernel/x_preact/detail"
import { ModifyFieldHandler } from "../../../../z_lib/ui/modify/action"
import { FocusAction } from "../../../../z_lib/ui/list/action"

import { AuthUserAccount } from "../../../../auth/user/account/kernel/data"

export function newDetailAuthUserAccountActions(
    feature: ForegroundOutsideFeature,
    focused: FocusAction<AuthUserAccount, unknown>,
): DetailAuthUserAccountActions {
    const modify = newModifyAuthUserAccountAction(feature)
    const changeResetTokenDestination = newChangeResetTokenDestinationAction(feature)
    const overwriteLoginId = newOverwriteLoginIdAction(feature)
    const overwritePassword = newOverwritePasswordAction(feature)
    const unregister = newUnregisterAuthUserAccountAction(feature)

    focused.onModify(
        buildHandler({
            modify: [modify, changeResetTokenDestination, overwriteLoginId, overwritePassword],
            unregister: [unregister],
        }),
    )

    return {
        modify: modify.action,
        changeResetTokenDestination: changeResetTokenDestination.action,
        overwriteLoginId: overwriteLoginId.action,
        overwritePassword: overwritePassword.action,
        unregister: unregister.action,
    }

    type Action<T, H> = Readonly<{
        action: {
            onSuccess(handler: H): void
        }
        handler: ModifyFieldHandler<T>
    }>
    function buildHandler<T>(
        actions: Readonly<{
            modify: Action<T, (data: Partial<T>) => void>[]
            unregister: Action<T, () => void>[]
        }>,
    ): ModifyFieldHandler<T> {
        actions.modify.forEach(({ action }) => {
            action.onSuccess((data) => focused.update(data))
        })
        actions.unregister.forEach(({ action }) => {
            action.onSuccess(() => focused.remove())
        })

        return {
            focus: (data: T) => {
                invoke("focus", (handler) => handler(data))
            },
            update: (data: T) => {
                invoke("update", (handler) => handler(data))
            },
            close: () => {
                invoke("close", (handler) => handler())
            },
        }

        function invoke<K extends keyof ModifyFieldHandler<T>>(
            method: K,
            invoker: (handler: ModifyFieldHandler<T>[K]) => void,
        ): void {
            actions.modify.forEach(({ handler }) => {
                invoker(handler[method])
            })
            actions.unregister.forEach(({ handler }) => {
                invoker(handler[method])
            })
        }
    }
}
