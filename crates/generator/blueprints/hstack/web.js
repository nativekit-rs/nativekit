export default function HStack(node) {
    const p = node.props || {};
    const isHdr = Boolean(p.is_header_bar);
    const widthProp = p.width || "";
    const alignX = p.align_x || "left";
    const alignY = p.align_y || "center";
    const isWrap = Boolean(p.wrap);
    const isSpaceBetween = alignX === "space_between" || (isHdr && (node.children || []).length === 2);
    const hasFlex = (node.children || []).some(c => {
        const cw = (c.props && c.props.width) || "";
        return cw === "fill" || (c.name === "spacer" && !((c.props && c.props.size) > 0)) || (c.name === "vstack" && cw !== "hug");
    });
    const isFill = widthProp === "fill" || (widthProp !== "hug" && (hasFlex || isSpaceBetween));
    const el = document.createElement("div");
    el.classList.add("hstack");
    applyWebContainerStyle(el, node);
    el.style.width = isFill ? "100%" : "fit-content";
    el.style.flexWrap = isWrap ? "wrap" : "nowrap";
    el.style.alignItems = alignY === "top" ? "flex-start" : (alignY === "bottom" ? "flex-end" : "center");
    el.style.justifyContent = isSpaceBetween ? "space-between" : (alignX === "center" ? "center" : (alignX === "right" ? "flex-end" : "flex-start"));
    if (node.children) {
        node.children.forEach(c => {
            const childEl = renderNode(c);
            const cw = (c.props && c.props.width) || "";
            if (cw === "fill" || (c.name === "vstack" && cw !== "hug" && !isSpaceBetween)) {
                childEl.style.flex = "1 1 0%";
            }
            el.appendChild(childEl);
        });
    }
    return el;
}
