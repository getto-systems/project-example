import { h, render } from "preact"

import { LogoutPageEntry } from "./page"
import { newLogoutPageView } from "./resource"

render(h(LogoutPageEntry, newLogoutPageView()), document.body)
