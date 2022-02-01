import { toApplicationView } from "../../../../../ui/vendor/getto-application/action/helper"

import { newCheckAuthTicketConfig } from "./config"

import { newGetScriptPathShell } from "../../../sign/get_script_path/init/infra"
import { newAuthTicketRepository } from "../../kernel/init/ticket_repository"
import { newCheckAuthTicketRemote } from "./check_remote"
import { newClock } from "../../../../z_lib/ui/clock/init"

import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"
import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"

import { CheckAuthTicketAction, initCheckAuthTicketAction } from "../action"
import { ApplicationView } from "../../../../../ui/vendor/getto-application/action/action"

type OutsideFeature = RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature
export function newCheckAuthTicketView(
    feature: OutsideFeature,
): ApplicationView<CheckAuthTicketAction> {
    return toApplicationView(
        initCheckAuthTicketAction({
            infra: {
                ticketRepository: newAuthTicketRepository(feature),
                renewRemote: newCheckAuthTicketRemote(feature, newClock()),
                clock: newClock(),
            },
            shell: {
                ...newGetScriptPathShell(feature),
            },
            config: newCheckAuthTicketConfig(),
        }),
    )
}
