import { h, render } from "preact"

import { foregroundOutsideFeature } from "../../../../../src/x_outside_feature/_ui/common"

import { newProfileView } from "../../../../../src/auth/_ui/action_profile/init"

import { ProfileEntry } from "../../../../../src/auth/_ui/action_profile/x_preact/profile"

render(h(ProfileEntry, newProfileView(foregroundOutsideFeature())), document.body)
