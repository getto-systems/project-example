import { newResetPasswordConfig } from "./config"

import { newGetScriptPathShell } from "../../../../../sign/get_script_path/init/infra"
import { toURL } from "../../../../../../common/util/location/init"
import { newClock } from "../../../../../../common/util/clock/init"
import { newAuthTicketRepository } from "../../../../../ticket/kernel/init/ticket_repository"
import { newCheckAuthTicketRemote } from "../../../../../ticket/authenticate/init/check_remote"
import { newResetPasswordRemote } from "./reset_remote"

import { detectResetToken } from "../convert"

import { RepositoryOutsideFeature } from "../../../../../../common/util/repository/feature"
import { LocationOutsideFeature } from "../../../../../../common/util/location/feature"

import { initResetPasswordAction, ResetPasswordAction } from "../action"

type OutsideFeature = RepositoryOutsideFeature & LocationOutsideFeature
export function newResetPasswordAction(feature: OutsideFeature): ResetPasswordAction {
    return initResetPasswordAction({
        infra: {
            ticketRepository: newAuthTicketRepository(feature),
            renewRemote: newCheckAuthTicketRemote(newClock()),
            resetRemote: newResetPasswordRemote(newClock()),
            clock: newClock(),
        },
        shell: {
            ...newGetScriptPathShell(feature),
            detectResetToken: () => detectResetToken(toURL(feature)),
        },
        config: newResetPasswordConfig(),
    })
}
