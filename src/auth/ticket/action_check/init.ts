import { ApplicationAbstractStateAction } from "../../../../ui/vendor/getto-application/action/init"

import {
    CheckAuthTicketAction,
    CheckAuthTicketMaterial,
    CheckAuthTicketState,
    initialCheckAuthTicketState,
} from "./action"

import { getScriptPath } from "../../sign/get_script_path/method"
import { startContinuousRenew, saveAuthTicket } from "../start_continuous_renew/method"
import { renewAuthTicket, checkAuthTicket } from "../check/method"

import { GetScriptPathInfra, GetScriptPathDetecter } from "../../sign/get_script_path/infra"
import { StartContinuousRenewInfra } from "../start_continuous_renew/infra"
import { CheckAuthTicketInfra } from "../check/infra"

import { AuthTicket } from "../kernel/data"
import { LoadScriptError } from "../../sign/get_script_path/data"

export type CheckAuthTicketActionInfra = Readonly<{
    check: CheckAuthTicketInfra
    startContinuousRenew: StartContinuousRenewInfra
    getSecureScriptPath: GetScriptPathInfra
}>

export function initCheckAuthTicketMaterial(
    infra: CheckAuthTicketActionInfra,
): CheckAuthTicketMaterial {
    return {
        renew: checkAuthTicket(infra.check),
        forceRenew: renewAuthTicket(infra.check),
        startContinuousRenew: startContinuousRenew(infra.startContinuousRenew),
        save: saveAuthTicket(infra.startContinuousRenew),
        getSecureScriptPath: getScriptPath(infra.getSecureScriptPath),
    }
}

export function initCheckAuthTicketAction(
    material: CheckAuthTicketMaterial,
    detecter: GetScriptPathDetecter,
): CheckAuthTicketAction {
    return new Action(material, detecter)
}

class Action
    extends ApplicationAbstractStateAction<CheckAuthTicketState>
    implements CheckAuthTicketAction
{
    readonly initialState = initialCheckAuthTicketState

    material: CheckAuthTicketMaterial
    detecter: GetScriptPathDetecter

    constructor(material: CheckAuthTicketMaterial, detecter: GetScriptPathDetecter) {
        super(async () =>
            this.material.renew((event) => {
                switch (event.type) {
                    case "try-to-instant-load":
                        return this.post({
                            type: "try-to-instant-load",
                            scriptPath: this.secureScriptPath(),
                        })

                    case "succeed-to-renew":
                        return this.startContinuousRenew(event.auth)

                    default:
                        return this.post(event)
                }
            }),
        )
        this.material = material
        this.detecter = detecter
    }

    succeedToInstantLoad(): Promise<CheckAuthTicketState> {
        return this.material.startContinuousRenew(this.post)
    }
    async failedToInstantLoad(): Promise<CheckAuthTicketState> {
        return this.material.forceRenew((event) => {
            switch (event.type) {
                case "succeed-to-renew":
                    return this.startContinuousRenew(event.auth)

                default:
                    return this.post(event)
            }
        })
    }
    async loadError(err: LoadScriptError): Promise<CheckAuthTicketState> {
        return this.post({ type: "load-error", err })
    }

    secureScriptPath() {
        return this.material.getSecureScriptPath(this.detecter())
    }

    async startContinuousRenew(info: AuthTicket): Promise<CheckAuthTicketState> {
        return this.material.save(info, (event) => {
            switch (event.type) {
                case "failed-to-save":
                    return this.post({ type: "repository-error", err: event.err })

                case "succeed-to-save":
                    return this.material.startContinuousRenew((event) => {
                        switch (event.type) {
                            case "succeed-to-start-continuous-renew":
                                return this.post({
                                    type: "try-to-load",
                                    scriptPath: this.secureScriptPath(),
                                })

                            default:
                                return this.post(event)
                        }
                    })
            }
        })
    }
}
