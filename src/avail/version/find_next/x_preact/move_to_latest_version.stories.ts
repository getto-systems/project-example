import { h } from "preact"

import { storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { MoveToLatestVersionComponent } from "./move_to_latest_version"

import { newFindNextVersionConfig } from "../init/config"
import { mockRemoteInfraError } from "../../../../z_lib/ui/remote/mock"
import { mockFindNextVersionShell } from "../init/mock"

import { FindNextVersionState } from "../action"
import { initFindNextVersionAction } from "../../find_next/action"

const options = ["takeLongtime", "failed"] as const

export default {
    title: "main/Avail/Move To Latest Version",
    parameters: {
        layout: "fullscreen",
    },
    argTypes: {
        findNext: {
            control: { type: "select", options },
        },
    },
}

type MockProps = Readonly<{
    findNext: typeof options[number]
    err: string
}>
const template = storyTemplate<MockProps>((props) => {
    return h(MoveToLatestVersionComponent, {
        findNext: initFindNextVersionAction(
            newFindNextVersionConfig(),
            {
                check: async () => mockRemoteInfraError,
                version: "0.0.0",
                versionSuffix: "-ui",
            },
            mockFindNextVersionShell(new URL("https://example.com"), "0.0.0"),
        ),
        state: state(),
    })

    function state(): FindNextVersionState {
        switch (props.findNext) {
            case "takeLongtime":
                return { type: "take-longtime-to-find" }

            case "failed":
                return {
                    type: "failed-to-find",
                    err: { type: "infra-error", err: props.err },
                }
        }
    }
})

export const MoveToLatestVersion = template({ findNext: "takeLongtime", err: "" })
