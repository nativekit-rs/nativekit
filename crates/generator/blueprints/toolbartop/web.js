export default function ToolbarTop(node) {
    return h("div", { className: "toolbartop", style: { display: "inline-flex", alignItems: "center", gap: "6px" } }, renderChildren(node));
}
