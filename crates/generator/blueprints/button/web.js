export default function Button(node) {
    const { variant = "", action = "", color = "", opacity = 100, width = "" } = node.props || {};
    const isFill = width === "fill";
    const el = h("button", {
        className: variant === "primary" ? "primary" : (variant === "secondary" ? "secondary" : "text"),
        fgColor: (color && variant !== "primary") ? color : "",
        bgColor: (color && variant === "primary") ? color : "",
        style: {
            width: isFill ? "100%" : "fit-content",
            alignSelf: isFill ? "stretch" : "flex-start",
            flex: isFill ? "1" : "none",
            opacity: opacity < 100 ? String(opacity / 100.0) : "",
        },
        onClick: () => { if (el.__action) dispatch(el.__action, null); },
    }, renderChildren(node));
    el.__action = action;
    el.__hasListener = true;
    return el;
}
