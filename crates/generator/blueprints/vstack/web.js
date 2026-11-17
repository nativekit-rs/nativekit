export default function VStack(node) {
    const p = node.props || {};
    const widthProp = p.width || "";
    const alignX = p.align_x || "left";
    const alignY = p.align_y || "top";
    const isFill = widthProp !== "hug";
    const el = document.createElement("div");
    el.classList.add("vstack");
    applyWebContainerStyle(el, node);
    el.style.width = isFill ? "100%" : "fit-content";
    el.style.alignItems = alignX === "center" ? "center" : (alignX === "right" ? "flex-end" : "stretch");
    el.style.justifyContent = alignY === "space_between" ? "space-between" : (alignY === "center" ? "center" : (alignY === "bottom" ? "flex-end" : "flex-start"));
    if (node.children) node.children.forEach(c => el.appendChild(renderNode(c)));
    return el;
}
