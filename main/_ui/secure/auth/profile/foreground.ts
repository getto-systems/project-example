import { h, render } from "preact"

import { newForegroundOutsideFeature } from "../../../../../src/x_outside_feature/_ui/common"

import { newProfileView } from "../../../../../src/auth/_ui/action_profile/init/resource"

import { ProfileEntry } from "../../../../../src/auth/_ui/action_profile/x_preact/profile"

render(h(ProfileEntry, newProfileView(newForegroundOutsideFeature())), document.body)
