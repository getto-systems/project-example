import { toApplicationView } from "../../../../../../../ui/vendor/getto-application/action/helper"

import { newResetPasswordConfig } from "./config"

import { newGetScriptPathShell } from "../../../../../sign/get_script_path/init/infra"
import { toURL } from "../../../../../../z_lib/ui/location/init"
import { newClock } from "../../../../../../z_lib/ui/clock/init"
import { newAuthTicketRepository } from "../../../../../ticket/kernel/init/ticket_repository"
import { newRenewAuthTicketRemote } from "../../../../../ticket/kernel/init/renew_remote"
import { newResetPasswordRemote } from "./reset_remote"

import { detectResetToken } from "../../../input/convert"

import { RemoteOutsideFeature } from "../../../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../../z_lib/ui/repository/feature"
import { LocationOutsideFeature } from "../../../../../../z_lib/ui/location/feature"

import { initResetPasswordAction, ResetPasswordAction } from "../action"
import { ApplicationView } from "../../../../../../../ui/vendor/getto-application/action/action"

export function newResetPasswordView(
    feature: RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature,
): ApplicationView<ResetPasswordAction> {
    return toApplicationView(
        initResetPasswordAction(
            newResetPasswordConfig(),
            {
                ticketRepository: newAuthTicketRepository(feature),
                renewRemote: newRenewAuthTicketRemote(feature, newClock()),
                resetRemote: newResetPasswordRemote(feature, newClock()),
                clock: newClock(),
            },
            {
                ...newGetScriptPathShell(feature),
                detectResetToken: () => detectResetToken(toURL(feature)),
            },
        ),
    )
}
