import { newCheckResetTokenSendingStatusLocationDetecter } from "../../../check_status/init"

import { initCheckResetTokenSendingStatusView } from "../../impl"
import { initCheckResetTokenSendingStatusCoreAction } from "../../core/impl"

import {
    WorkerAbstractProxy,
    WorkerProxy,
} from "../../../../../../../../ui/vendor/getto-application/action/worker/foreground"

import {
    CheckPasswordResetSendingStatusProxyMaterial,
    CheckPasswordResetSendingStatusProxyMessage,
    CheckPasswordResetSendingStatusProxyResponse,
} from "./message"

import { CheckResetTokenSendingStatusView } from "../../resource"

import { LocationOutsideFeature } from "../../../../../../../z_details/_ui/location/feature"

type OutsideFeature = LocationOutsideFeature
export interface CheckPasswordResetSendingStatusProxy
    extends WorkerProxy<
        CheckPasswordResetSendingStatusProxyMessage,
        CheckPasswordResetSendingStatusProxyResponse
    > {
    view(feature: OutsideFeature): CheckResetTokenSendingStatusView
}
export function newCheckPasswordResetSendingStatusProxy(
    post: Post<CheckPasswordResetSendingStatusProxyMessage>,
): CheckPasswordResetSendingStatusProxy {
    return new Proxy(post)
}

class Proxy
    extends WorkerAbstractProxy<
        CheckPasswordResetSendingStatusProxyMessage,
        CheckPasswordResetSendingStatusProxyResponse
    >
    implements CheckPasswordResetSendingStatusProxy {
    material: CheckPasswordResetSendingStatusProxyMaterial

    constructor(post: Post<CheckPasswordResetSendingStatusProxyMessage>) {
        super(post)
        this.material = {
            checkStatus: this.method("checkStatus", (message) => message),
        }
    }

    view(feature: OutsideFeature): CheckResetTokenSendingStatusView {
        const detecter = newCheckResetTokenSendingStatusLocationDetecter(feature)
        return initCheckResetTokenSendingStatusView(
            initCheckResetTokenSendingStatusCoreAction({
                checkStatus: (post) => this.material.checkStatus.call(detecter(), post),
            }),
        )
    }
    resolve(response: CheckPasswordResetSendingStatusProxyResponse): void {
        switch (response.method) {
            case "checkStatus":
                this.material.checkStatus.resolve(response)
                break
        }
    }
}

interface Post<M> {
    (message: M): void
}
