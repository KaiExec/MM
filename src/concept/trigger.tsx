import { SetStateAction, Dispatch } from 'react';
import { invoke } from '@tauri-apps/api/core'

import { DTO } from './utils.tsx'

export async function trigger(
    name: keyof DTO,
    triggerState: boolean,
    trigger: Dispatch<SetStateAction<boolean>>,
) {
    // Store
    if (name == "side_effect") {
        invoke("trigger_side_effect", { isExpand: triggerState })
    }

    // Trigger
    trigger(!triggerState)
}
