import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"
import { toApplicationView } from "../../../../../ui/vendor/getto-application/action/helper"

import { newCheckAuthTicketInfra } from "../check/init"
import { newStartContinuousRenewAuthnInfoInfra } from "../start_continuous_renew/init"
import { newGetSecureScriptPathInfra } from "../../../_ui/common/secure/get_script_path/init"
import { newGetScriptPathLocationDetecter } from "../../../_ui/common/secure/get_script_path/init"

import { RemoteOutsideFeature } from "../../../../z_details/_ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../z_details/_ui/repository/feature"
import { LocationOutsideFeature } from "../../../../z_details/_ui/location/feature"

import { CheckAuthTicketView } from "./resource"
import {
    CheckAuthTicketAction,
    CheckAuthTicketMaterial,
    CheckAuthTicketState,
    initialCheckAuthTicketState,
} from "./action"

import {
    GetScriptPathDetecter,
    getScriptPath,
} from "../../../_ui/common/secure/get_script_path/method"
import { startContinuousRenew, saveAuthTicket } from "../start_continuous_renew/method"
import { renewAuthTicket, checkAuthTicket } from "../check/method"

import { GetScriptPathInfra } from "../../../_ui/common/secure/get_script_path/infra"
import { StartContinuousRenewInfra } from "../start_continuous_renew/infra"
import { CheckAuthTicketInfra } from "../check/infra"

import { AuthTicket } from "../kernel/data"
import { LoadScriptError } from "../../../_ui/common/secure/get_script_path/data"

type OutsideFeature = RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature
export function newCheckAuthTicketView(feature: OutsideFeature): CheckAuthTicketView {
    return toApplicationView(
        initCheckAuthTicketAction(
            initCheckAuthTicketMaterial(
                {
                    check: newCheckAuthTicketInfra(feature),
                    startContinuousRenew: newStartContinuousRenewAuthnInfoInfra(feature),
                    getSecureScriptPath: newGetSecureScriptPathInfra(),
                },
                newGetScriptPathLocationDetecter(feature),
            ),
        ),
    )
}

export type CheckAuthTicketActionInfra = Readonly<{
    check: CheckAuthTicketInfra
    startContinuousRenew: StartContinuousRenewInfra
    getSecureScriptPath: GetScriptPathInfra
}>

export function initCheckAuthTicketMaterial(
    infra: CheckAuthTicketActionInfra,
    detecter: GetScriptPathDetecter,
): CheckAuthTicketMaterial {
    return {
        renew: checkAuthTicket(infra.check),
        forceRenew: renewAuthTicket(infra.check),
        startContinuousRenew: startContinuousRenew(infra.startContinuousRenew),
        save: saveAuthTicket(infra.startContinuousRenew),
        getSecureScriptPath: getScriptPath(infra.getSecureScriptPath)(detecter),
    }
}

export function initCheckAuthTicketAction(
    material: CheckAuthTicketMaterial,
): CheckAuthTicketAction {
    return new Action(material)
}

class Action
    extends ApplicationAbstractStateAction<CheckAuthTicketState>
    implements CheckAuthTicketAction
{
    readonly initialState = initialCheckAuthTicketState

    material: CheckAuthTicketMaterial

    constructor(material: CheckAuthTicketMaterial) {
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
        return this.material.getSecureScriptPath()
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
