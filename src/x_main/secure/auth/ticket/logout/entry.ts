import { h, render } from "preact"

import { LogoutPage } from "./page"
import { newLogoutPageResource } from "./resource"

render(h(LogoutPage, newLogoutPageResource()), document.body)
