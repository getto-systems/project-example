import { toApplicationView } from "../../../../../../z_vendor/getto-application/action/helper"

import { newResetPasswordConfig } from "./config"

import { newGetScriptPathShell } from "../../../../../sign/get_script_path/init/infra"
import { toURL } from "../../../../../../z_lib/ui/location/init"
import { newClock } from "../../../../../../z_lib/ui/clock/init"
import { newAuthTicketRepository } from "../../../../../ticket/kernel/init/ticket_repository"
import { newCheckAuthTicketRemote } from "../../../../../ticket/check/init/check_remote"
import { newResetPasswordRemote } from "./reset_remote"

import { detectResetToken } from "../../../input/convert"

import { RemoteOutsideFeature } from "../../../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../../z_lib/ui/repository/feature"
import { LocationOutsideFeature } from "../../../../../../z_lib/ui/location/feature"

import { initResetPasswordAction, ResetPasswordAction } from "../action"
import { ApplicationView } from "../../../../../../z_vendor/getto-application/action/action"

export function newResetPasswordView(
    feature: RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature,
): ApplicationView<ResetPasswordAction> {
    return toApplicationView(
        initResetPasswordAction({
            infra: {
                ticketRepository: newAuthTicketRepository(feature),
                renewRemote: newCheckAuthTicketRemote(feature, newClock()),
                resetRemote: newResetPasswordRemote(feature, newClock()),
                clock: newClock(),
            },
            shell: {
                ...newGetScriptPathShell(feature),
                detectResetToken: () => detectResetToken(toURL(feature)),
            },
            config: newResetPasswordConfig(),
        }),
    )
}
