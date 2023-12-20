import { PreactContent } from "../../../../../../../common/x_preact/vnode"
import { ResetTokenDestinationType } from "../data"

export function resetTokenDestinationTypeLabel(type: ResetTokenDestinationType): PreactContent {
    switch (type) {
        case "none":
            return "無効"

        case "email":
            return "有効"
    }
}
