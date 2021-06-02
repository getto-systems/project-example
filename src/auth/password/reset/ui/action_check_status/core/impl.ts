import { ApplicationAbstractStateAction } from "../../../../../../../ui/vendor/getto-application/action/impl"

import { checkSendingStatus } from "../../check_status/impl/core"

import { CheckResetTokenSendingStatusInfra } from "../../check_status/infra"

import {
    CheckResetTokenSendingStatusCoreAction,
    CheckResetTokenSendingStatusCoreMaterial,
    CheckResetTokenSendingStatusCoreMaterialPod,
    CheckResetTokenSendingStatusCoreState,
    initialCheckResetTokenSendingStatusCoreState,
} from "./action"

import { CheckResetTokenSendingStatusLocationDetecter } from "../../check_status/method"

export function initCheckResetTokenSendingStatusCoreMaterial(
    infra: CheckResetTokenSendingStatusInfra,
    detecter: CheckResetTokenSendingStatusLocationDetecter,
): CheckResetTokenSendingStatusCoreMaterial {
    const pod = initCheckResetTokenSendingStatusCoreMaterialPod(infra)
    return {
        checkStatus: pod.initCheckStatus(detecter),
    }
}
export function initCheckResetTokenSendingStatusCoreMaterialPod(
    infra: CheckResetTokenSendingStatusInfra,
): CheckResetTokenSendingStatusCoreMaterialPod {
    return {
        initCheckStatus: checkSendingStatus(infra),
    }
}

export function initCheckResetTokenSendingStatusCoreAction(
    material: CheckResetTokenSendingStatusCoreMaterial,
): CheckResetTokenSendingStatusCoreAction {
    return new Action(material)
}

class Action
    extends ApplicationAbstractStateAction<CheckResetTokenSendingStatusCoreState>
    implements CheckResetTokenSendingStatusCoreAction {
    readonly initialState = initialCheckResetTokenSendingStatusCoreState

    material: CheckResetTokenSendingStatusCoreMaterial

    constructor(material: CheckResetTokenSendingStatusCoreMaterial) {
        super(() => this.material.checkStatus(this.post))
        this.material = material
    }
}
