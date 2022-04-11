import { h, render } from "preact"

import { LogoutPage } from "./page"
import { newLogoutPageView } from "./resource"

render(h(LogoutPage, newLogoutPageView()), document.body)
