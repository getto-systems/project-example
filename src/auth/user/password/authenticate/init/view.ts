import { newAuthenticatePasswordConfig } from "./config"

import { newGetScriptPathShell } from "../../../../sign/get_script_path/init/infra"
import { newClock } from "../../../../../z_lib/ui/clock/init"
import { newAuthenticatePasswordRemote } from "./authenticate_remote"
import { newAuthTicketRepository } from "../../../../ticket/kernel/init/ticket_repository"
import { newCheckAuthTicketRemote } from "../../../../ticket/check/init/check_remote"

import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../z_lib/ui/repository/feature"
import { LocationOutsideFeature } from "../../../../../z_lib/ui/location/feature"

import { AuthenticatePasswordAction, initAuthenticatePasswordAction } from "../action"

export function newAuthenticatePasswordAction(
    feature: RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature,
): AuthenticatePasswordAction {
    return initAuthenticatePasswordAction({
        infra: {
            ticketRepository: newAuthTicketRepository(feature),
            renewRemote: newCheckAuthTicketRemote(feature, newClock()),
            authenticateRemote: newAuthenticatePasswordRemote(feature, newClock()),
            clock: newClock(),
        },
        shell: {
            ...newGetScriptPathShell(feature),
        },
        config: newAuthenticatePasswordConfig(),
    })
}
