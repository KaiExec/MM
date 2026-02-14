export interface DTO {
    motivation: string,
    means: string,
    side_effect: string
}

export const font = {
    font_web_path: "fonts/huiwen.ttf",
    font_family: "HuiWenFont"
}


export const GlobalFontStyle = () => {
    return (
        <style>{`
                 @font-face {
                   font-family: "${font.font_family}";
                   src: url("${font.font_web_path}") format("truetype");
                   font-display: swap;
                 }
                body {
                   font-family: "${font.font_family}", system-ui;
                    }
            `}</style>
    )
}
