import { toApplicationView } from "../../../../../../ui/vendor/getto-application/action/helper"

import { newAuthenticatePasswordConfig } from "./config"

import { newGetScriptPathShell } from "../../../../sign/get_script_path/init/infra"
import { newClock } from "../../../../../z_lib/ui/clock/init"
import { newAuthenticatePasswordRemote } from "./authenticate_remote"
import { newAuthTicketRepository } from "../../../../ticket/kernel/init/ticket_repository"
import { newRenewAuthTicketRemote } from "../../../../ticket/kernel/init/renew_remote"

import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../z_lib/ui/repository/feature"
import { LocationOutsideFeature } from "../../../../../z_lib/ui/location/feature"

import { ApplicationView } from "../../../../../../ui/vendor/getto-application/action/action"
import { AuthenticatePasswordAction, initAuthenticatePasswordAction } from "../action"

export function newAuthenticatePasswordView(
    feature: RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature,
): ApplicationView<AuthenticatePasswordAction> {
    return toApplicationView(
        initAuthenticatePasswordAction({
            infra: {
                ticketRepository: newAuthTicketRepository(feature),
                renewRemote: newRenewAuthTicketRemote(feature, newClock()),
                authenticateRemote: newAuthenticatePasswordRemote(feature, newClock()),
                clock: newClock(),
            },
            shell: {
                ...newGetScriptPathShell(feature),
            },
            config: newAuthenticatePasswordConfig(),
        }),
    )
}
