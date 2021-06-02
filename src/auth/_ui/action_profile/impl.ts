import { initBaseView } from "../../../example/_ui/action_base/impl"
import { ProfileResource, ProfileView } from "./resource"

export function initProfileView(resource: ProfileResource): ProfileView {
    return initBaseView(resource, () => {
        resource.logout.terminate()
    })
}
