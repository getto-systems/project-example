import { newCheckAuthTicketConfig } from "./config"

import { newGetScriptPathShell } from "../../../sign/get_script_path/init/infra"
import { newAuthTicketRepository } from "../../kernel/init/ticket_repository"
import { newCheckAuthTicketRemote } from "./check_remote"
import { newClock } from "../../../../common/util/clock/init"

import { RepositoryOutsideFeature } from "../../../../common/util/repository/feature"
import { LocationOutsideFeature } from "../../../../common/util/location/feature"

import { AuthenticateWithTokenAction, initAuthenticateWithTokenAction } from "../action"

type OutsideFeature = RepositoryOutsideFeature & LocationOutsideFeature
export function newCheckAuthTicketAction(feature: OutsideFeature): AuthenticateWithTokenAction {
    return initAuthenticateWithTokenAction({
        infra: {
            ticketRepository: newAuthTicketRepository(feature),
            renewRemote: newCheckAuthTicketRemote(newClock()),
            clock: newClock(),
        },
        shell: {
            ...newGetScriptPathShell(feature),
        },
        config: newCheckAuthTicketConfig(),
    })
}
