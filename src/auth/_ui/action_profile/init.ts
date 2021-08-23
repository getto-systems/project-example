import { initBaseView } from "../../../example/_ui/action_base/init"
import { ProfileResource, ProfileView } from "./resource"

export function initProfileView(resource: ProfileResource): ProfileView {
    return initBaseView(resource, () => {
        resource.logout.terminate()
    })
}
