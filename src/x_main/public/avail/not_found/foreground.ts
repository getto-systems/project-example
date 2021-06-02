import { render, h } from "preact"

import { newNotFoundView } from "../../../../avail/_ui/action_not_found/init"

import { NotFoundEntry } from "../../../../avail/_ui/action_not_found/x_preact/not_found"

render(h(NotFoundEntry, newNotFoundView()), document.body)
