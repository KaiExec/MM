import "./Styles/App.scss"
import { useState, useRef, useEffect } from 'react';
import { trigger } from "./concept/trigger.tsx";
import { invoke } from "@tauri-apps/api/core";
import { DTO, GlobalFontStyle } from './concept/utils.tsx'
import { TrayIcon, TrayIconOptions } from '@tauri-apps/api/tray';
import { defaultWindowIcon } from '@tauri-apps/api/app';
import { Menu } from '@tauri-apps/api/menu';

export default function App() {
    const [isExpand, setIsExpand] = useState(false)
    const [isSundry, setIsSundry] = useState(false)
    const [isSaveDefaultly, setIsSaveDefaultly] = useState(true)
    const [importTrigger, setImportTrigger] = useState(false);

    const motivationRef = useRef<HTMLHeadingElement>(null);
    const meansRef = useRef<HTMLParagraphElement>(null);
    const sideEffectRef = useRef<HTMLParagraphElement>(null);
    const content = useRef(
        {
            motivation: "",
            means: "",
            side_effect: ""
        } as DTO
    )


    useEffect(() => {
        if (sideEffectRef.current) {
            sideEffectRef.current.innerText = content.current.side_effect
        }
    }, [isExpand, importTrigger])

    useEffect(() => {
        if (meansRef.current) {
            meansRef.current.innerText = content.current.means
        }

    }, [isSundry, importTrigger])

    useEffect(() => {
        if (motivationRef.current) {
            motivationRef.current.innerText = content.current.motivation
        }
    }, [importTrigger])


    // Function
    function handleMeansContainer(e: React.MouseEvent<HTMLElement>) {
        if (meansRef.current && e.target !== meansRef.current) {
            (meansRef.current).focus();
        }
    };

    function handleSave(content: DTO, defaultly: boolean) {
        invoke("save", { content: content, defaultly: defaultly })
    }


    useEffect(() => {
        async function init() {
            let icon;
            const TRAY_ID = "my-tray"
            try {
                const icon_test = await defaultWindowIcon();
                if (icon_test) icon = icon_test; else return;
            } catch (err) {
                if (err instanceof Error) {
                    console.error("Request icon Failed", err.message);
                    return
                }
            }
            const menu = await Menu.new({
                items: [
                    {
                        id: 'quit',
                        text: 'Quit',
                        action: () => {
                            console.log('quit pressed');
                            invoke("exit_suc")
                        }
                    },
                ],
            });
            const options: TrayIconOptions = {
                id: TRAY_ID,
                icon: icon,
                menu,
                menuOnLeftClick: true,
            };
            await TrayIcon.new(options);
        }
        init()
    }, [])


    // JSX
    return (
        <>
            <GlobalFontStyle />

            <div id="drag-bar" data-tauri-drag-region></div>
            <div id="content">
                {isExpand && <p contentEditable={true}
                    suppressContentEditableWarning={true}
                    ref={sideEffectRef} id="sideEffect" onBlur={
                        () => {
                            if (sideEffectRef.current) {
                                content.current.side_effect = sideEffectRef.current.innerText || "";
                            }
                        }
                    }
                ></p>}
                <article id="main-container">
                    <div id="title-bar">
                        <button id="sideEffect-btn" onClick={
                            () => { trigger("side_effect", isExpand, setIsExpand) }
                        }>↞</button>
                        <h1 id="main-container-motivation"
                            contentEditable={true}

                            suppressContentEditableWarning={true}
                            data-placeholder={"Nothing"}
                            ref={motivationRef}
                            onBlur={
                                () => {
                                    if (motivationRef.current) {
                                        content.current.motivation = motivationRef.current.innerText || "";
                                    }
                                }
                            }
                        ></h1>
                        <button id="sundry-btn" onClick={
                            () => { trigger("means", isSundry, setIsSundry) }
                        }>⚙︎</button>
                    </div>
                    <hr />
                    {isSundry ? (
                        <div id="sundry">
                            <button onClick={
                                () => {
                                    invoke<string | null>("import").then(
                                        (res) => {
                                            if (res) {
                                                content.current = JSON.parse(res) as DTO
                                                setImportTrigger(!importTrigger)
                                            }
                                        }
                                    )
                                }
                            }>
                                Import
                            </button>
                            <div className="muliEle">
                                <button onClick={() => { handleSave(content.current, isSaveDefaultly) }}>
                                    Save
                                </button>
                                <div >
                                    <input
                                        name="default-save"
                                        type="checkbox"
                                        checked={isSaveDefaultly}
                                        onChange={(e) => setIsSaveDefaultly(e.target.checked)}
                                    />
                                    <label>Save defaultly</label>
                                </div>
                            </div>
                            <button>TBD</button>
                            <button>TBD</button>
                        </div>
                    ) : (
                        <div id="main-container-means-container" onClick={handleMeansContainer}>
                            <p id="main-container-means"
                                ref={meansRef}
                                contentEditable={true}
                                suppressContentEditableWarning={true}
                                onBlur={
                                    () => {
                                        if (meansRef.current) {
                                            content.current.means = meansRef.current.innerText || "";
                                        }
                                    }
                                }
                            ></p>
                        </div>
                    )}
                </article>
            </div>
        </>
    )
}
