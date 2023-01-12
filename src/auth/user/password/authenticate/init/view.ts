import { newAuthenticatePasswordConfig } from "./config"

import { newGetScriptPathShell } from "../../../../sign/get_script_path/init/infra"
import { newClock } from "../../../../../common/util/clock/init"
import { newAuthenticatePasswordRemote } from "./authenticate_remote"
import { newAuthTicketRepository } from "../../../../ticket/kernel/init/ticket_repository"
import { newCheckAuthTicketRemote } from "../../../../ticket/authenticate/init/check_remote"

import { RepositoryOutsideFeature } from "../../../../../common/util/repository/feature"
import { LocationOutsideFeature } from "../../../../../common/util/location/feature"

import { AuthenticatePasswordAction, initAuthenticatePasswordAction } from "../action"

export function newAuthenticatePasswordAction(
    feature: RepositoryOutsideFeature & LocationOutsideFeature,
): AuthenticatePasswordAction {
    return initAuthenticatePasswordAction({
        infra: {
            ticketRepository: newAuthTicketRepository(feature),
            renewRemote: newCheckAuthTicketRemote(newClock()),
            authenticateRemote: newAuthenticatePasswordRemote(newClock()),
            clock: newClock(),
        },
        shell: {
            ...newGetScriptPathShell(feature),
        },
        config: newAuthenticatePasswordConfig(),
    })
}
