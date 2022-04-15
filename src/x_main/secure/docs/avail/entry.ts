import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../x_outside_feature/common"
import { newDocsView } from "../../../../docs/init/resource"

import { Docs } from "../../../../docs/x_preact/docs"

import { docs_avail } from "../../../../avail/docs"
import { docs_avail_unexpectedError } from "../../../../avail/unexpected_error/docs"
import { docs_avail_version } from "../../../../avail/version/docs"

render(
    h(Docs, {
        view: newDocsView(newForegroundOutsideFeature()),
        title: "保守・運用",
        docs: [docs_avail, docs_avail_version, docs_avail_unexpectedError],
    }),
    document.body,
)
