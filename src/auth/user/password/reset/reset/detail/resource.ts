import { newResetPasswordConfig } from "./config"

import { newGetScriptPathShell } from "../../../../../sign/get_script_path/detail/infra"
import { toURL } from "../../../../../../common/util/location/detail"
import { newClock } from "../../../../../../common/util/clock/detail"
import { newAuthTicketRepository } from "../../../../../ticket/kernel/detail/ticket_repository"
import { newCheckAuthTicketRemote } from "../../../../../ticket/authenticate/detail/check_remote"
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
